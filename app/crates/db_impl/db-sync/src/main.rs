extern crate openssl;
#[macro_use]
extern crate diesel_migrations;

use aws_sdk_secretsmanager::Region;
use diesel::{connection::Connection, prelude::*};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use failure::Error;
use lambda_runtime::{service_fn, LambdaEvent};
use log::debug;
use serde_derive::Deserialize;
use serde_json::{json, Value};

use crate::diesel_migrations::MigrationHarness;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct UserParameters {
    credentialsParameterName: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await.unwrap();
    Ok(())
}

pub(crate) async fn my_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    println!("lambda event value = {}", event.payload);
    let shared_config = aws_config::load_from_env().await;
    let config = aws_sdk_ssm::config::Builder::from(&shared_config)
        .region(Some(Region::from_static("us-east-1")))
        .build();
    let client = aws_sdk_ssm::Client::from_conf(config);

    let code_pipeline_job = &event.payload["CodePipeline.job"];
    let event_data = &code_pipeline_job["data"];
    let action_configuration = &event_data["actionConfiguration"];
    let configuration = &action_configuration["configuration"];
    let user_params = &configuration["UserParameters"];
    let mut user_parameters = format!("{}", user_params);
    user_parameters.pop();
    user_parameters.remove(0);
    let user_parameters = user_parameters.replace('\\', "");
    let user_params: UserParameters = serde_json::from_str(&user_parameters).unwrap();
    println!("user_params = {:?}", user_params);

    let credentials_arn = client
        .get_parameter()
        .name(user_params.credentialsParameterName)
        .send()
        .await
        .unwrap();

    let config = aws_sdk_secretsmanager::config::Builder::from(&shared_config)
        .region(Some(Region::from_static("us-east-1")))
        .build();
    let client = aws_sdk_secretsmanager::Client::from_conf(config);
    let db_credentials = client
        .get_secret_value()
        .secret_id(credentials_arn.parameter.unwrap().value.unwrap())
        .send()
        .await
        .unwrap();

    let json_db_creds: serde_json::Value = serde_json::from_str(db_credentials.secret_string().unwrap()).unwrap();

    println!("Db Credentials: {:?}", db_credentials);
    let db_url = format!(
        "postgres://postgres:{}@{}:5432",
        json_db_creds.get("password").unwrap().as_str().unwrap(),
        json_db_creds.get("host").unwrap().as_str().unwrap()
    );

    println!("db_url = {}", db_url);

    debug!("Waiting for database...");
    let mut conn: PgConnection = Connection::establish(&db_url).unwrap();
    debug!("Database connected");
    conn.run_pending_migrations(MIGRATIONS).unwrap();
    debug!("Database migrated");

    let shared_config = aws_config::load_from_env().await;
    let config = aws_sdk_codepipeline::config::Builder::from(&shared_config)
        .region(Some(Region::from_static("us-east-1")))
        .build();
    let client = aws_sdk_codepipeline::Client::from_conf(config);
    println!("job_id = {}", event.payload["CodePipeline.job"]["id"].as_str().unwrap());

    let result = client
        .put_job_success_result()
        .set_job_id(Some(event.payload["CodePipeline.job"]["id"].as_str().unwrap().to_string()))
        .send()
        .await
        .unwrap();
    println!("put_job_success_result = {:?}", result);

    Ok(json!(""))
}
