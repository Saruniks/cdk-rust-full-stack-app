mod auth;

use std::{env, marker::PhantomData, net::SocketAddr};

use async_trait::async_trait;
use auth::MyAuth0Authenticator;
use bb8::Pool;
use bb8_diesel::DieselConnectionManager;
use db_common::get_db_url;
use db_impl::{get_member_pair, get_voting_results, post_pair_vote, post_voting_results};
use diesel::PgConnection;
use dotenv::dotenv;
use openapi_client::{
    models::{AuthInfo, PostPairVoteRequest},
    server::MakeService,
    Api, GetAuthInfoResponse, GetHealthResponse, GetMemberPairResponse, GetVotingResultsResponse, PostPairVoteResponse,
    PostVotingResultsResponse,
};
use swagger::{ApiError, Authorization, EmptyContext, Has, XSpanIdString};

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting server v0.0.4");

    let db_url = get_db_url().await;
    let manager: DieselConnectionManager<PgConnection> = DieselConnectionManager::<PgConnection>::new(db_url);
    let pool = bb8::Pool::builder()
        .max_size(10)
        .test_on_check_out(true)
        .build(manager)
        .await
        .expect("Could not build connection pool");

    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let addr = format!("0.0.0.0:{}", server_port)
        .parse::<SocketAddr>()
        .expect("Failed to parse bind address");

    let auth_info = AuthInfo {
        auth0_domain: env::var("AUTH0_DOMAIN").unwrap(),
        auth0_client_id: env::var("AUTH0_CLIENT_ID").unwrap(),
    };

    let server = Server::new(auth_info, pool);

    let service = MakeService::new(server);

    let auth0_authority = env::var("AUTH0_AUTHORITY").expect("AUTH0_AUTHORITY must be set");

    let service = MyAuth0Authenticator::new(service, auth0_authority);

    let service = openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(service);

    hyper::server::Server::bind(&addr).serve(service).await.unwrap()
}

#[derive(Clone)]
pub struct Server<C> {
    auth_info: AuthInfo,
    pool: Pool<DieselConnectionManager<PgConnection>>,
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new(auth_info: AuthInfo, pool: Pool<DieselConnectionManager<PgConnection>>) -> Self {
        Server {
            auth_info,
            pool,
            marker: PhantomData,
        }
    }
}

#[async_trait]
impl<C> Api<C> for Server<C>
where
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync,
{
    async fn get_auth_info(&self, _context: &C) -> Result<GetAuthInfoResponse, ApiError> {
        println!("get_auth_info()");
        Ok(GetAuthInfoResponse::SuccessfulResponse(self.auth_info.clone()))
    }

    async fn get_health(&self, _context: &C) -> Result<GetHealthResponse, ApiError> {
        println!("get_health()");
        Ok(GetHealthResponse::SuccessfulResponse("Hello from Rust App".to_string()))
    }

    async fn get_member_pair(&self, context: &C) -> Result<GetMemberPairResponse, ApiError> {
        println!("get_member_pair()");
        let subject = Has::<Option<Authorization>>::get(context)
            .as_ref()
            .unwrap()
            .subject
            .clone()
            .to_string();

        Ok(GetMemberPairResponse::SuccessfulResponse(
            get_member_pair(self.pool.get().await.unwrap(), Some(subject)).await.unwrap(),
        ))
    }

    async fn get_voting_results(&self, page: Option<i32>, limit: Option<i32>, context: &C) -> Result<GetVotingResultsResponse, ApiError> {
        println!("get_voting_results({:?}, {:?})", page, limit);
        let subject = Has::<Option<Authorization>>::get(context)
            .as_ref()
            .map(|auth_data| auth_data.subject.clone())
            .clone();

        Ok(GetVotingResultsResponse::SuccessfulResponse(
            get_voting_results(self.pool.get().await.unwrap(), subject, page, limit)
                .await
                .unwrap(),
        ))
    }

    async fn post_pair_vote(&self, post_pair_vote_request: PostPairVoteRequest, context: &C) -> Result<PostPairVoteResponse, ApiError> {
        println!("post_pair_vote({:?})", post_pair_vote_request);
        let subject = Has::<Option<Authorization>>::get(context)
            .as_ref()
            .map(|auth_data| auth_data.subject.clone())
            .clone();

        post_pair_vote(self.pool.get().await.unwrap(), subject, post_pair_vote_request)
            .await
            .unwrap();
        Ok(PostPairVoteResponse::SuccessfulResponse)
    }

    async fn post_voting_results(&self, context: &C) -> Result<PostVotingResultsResponse, ApiError> {
        println!("post_voting_results()");
        let subject = Has::<Option<Authorization>>::get(context)
            .as_ref()
            .map(|auth_data| auth_data.subject.clone())
            .clone();

        post_voting_results(self.pool.get().await.unwrap(), subject).await.unwrap();
        Ok(PostVotingResultsResponse::SuccessfulResponse)
    }
}
