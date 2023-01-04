use std::env;

use anyhow::Result;
use aws_sdk_secretsmanager::Region;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;

pub async fn get_db_url() -> String {
    if env::var("LOCAL_DEVELOPMENT").is_ok() {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    } else {
        let shared_config = aws_config::load_from_env().await;
        let config = aws_sdk_ssm::config::Builder::from(&shared_config)
            .region(Some(Region::from_static("us-east-1")))
            .build();
        let client = aws_sdk_ssm::Client::from_conf(config);

        let credentials_param_name = env::var("CREDENTIALS_PARAMETER_NAME").unwrap();

        let credentials_arn = client.get_parameter().name(credentials_param_name).send().await.unwrap();

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
        format!(
            "postgres://postgres:{}@{}:5432",
            json_db_creds.get("password").unwrap().as_str().unwrap(),
            json_db_creds.get("host").unwrap().as_str().unwrap()
        )
    }
}

pub async fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();
    let db_url = get_db_url().await;
    Ok(PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url)))
}
