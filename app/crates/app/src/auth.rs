use std::{
    marker::PhantomData,
    task::{Context, Poll},
};

use futures::FutureExt;
use hyper::{service::Service, Request};
use swagger::{auth::Scopes, Authorization, Push};

/// Bound for Request Context for MakeService wrappers
pub trait RcBound: Push<Option<Authorization>> + Send + 'static {}

impl<T> RcBound for T where T: Push<Option<Authorization>> + Send + 'static {}

/// Dummy Authenticator, that blindly inserts authorization data, allowing all
/// access to an endpoint with the specified subject.
#[derive(Debug)]
pub struct MyAuth0Authenticator<T, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
{
    inner: T,
    auth0_authority: String,
    marker: PhantomData<RC>,
}

impl<T, RC> MyAuth0Authenticator<T, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
{
    /// Create a middleware that authorizes with the configured subject.
    pub fn new(inner: T, auth0_authority: String) -> Self {
        MyAuth0Authenticator {
            inner,
            auth0_authority,
            marker: PhantomData,
        }
    }
}

impl<Inner, RC, Target> Service<Target> for MyAuth0Authenticator<Inner, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
    Inner: Service<Target>,
    Inner::Future: Send + 'static,
{
    type Error = Inner::Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = Auth0AccessTokenAuthenticator<Inner::Response, RC>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, target: Target) -> Self::Future {
        let auth0_authority = self.auth0_authority.clone();
        Box::pin(
            self.inner
                .call(target)
                .map(|s| Ok(Auth0AccessTokenAuthenticator::new(s?, auth0_authority))),
        )
    }
}

#[derive(Debug)]
pub struct Auth0AccessTokenAuthenticator<T, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
{
    inner: T,
    auth0_authority: String,
    marker: PhantomData<RC>,
}

impl<T, RC> Auth0AccessTokenAuthenticator<T, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
{
    /// Create a middleware that authorizes with the configured subject.
    pub fn new(inner: T, auth0_authority: String) -> Self {
        Auth0AccessTokenAuthenticator {
            inner,
            auth0_authority,
            marker: PhantomData,
        }
    }
}

impl<T, RC> Clone for Auth0AccessTokenAuthenticator<T, RC>
where
    T: Clone,
    RC: RcBound,
    RC::Result: Send + 'static,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            auth0_authority: self.auth0_authority.clone(),
            marker: PhantomData,
        }
    }
}

impl<T, B, RC> Service<(Request<B>, RC)> for Auth0AccessTokenAuthenticator<T, RC>
where
    RC: RcBound,
    RC::Result: Send + 'static,
    T: Service<(Request<B>, RC::Result)>,
{
    type Error = T::Error;
    type Future = T::Future;
    type Response = T::Response;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<B>, RC)) -> Self::Future {
        let (request, context) = req;

        // check request and parse scopes
        if let Some(auth_header) = request.headers().get("Authorization") {
            let token = auth_header.to_str().unwrap().chars().into_iter().skip(7).collect::<String>();
            let (_is_valid, subject, permissions) = validate_token(&token, self.auth0_authority.clone());

            let context = context.push(Some(Authorization {
                subject,
                scopes: Scopes::Some(permissions.into_iter().collect()),
                issuer: None,
            }));

            self.inner.call((request, context))
        } else {
            let context = context.push(None);
            self.inner.call((request, context))
        }
    }
}

// use crate::errors::ServiceError;
use std::error::Error;

use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
    permissions: Vec<String>,
}

pub fn validate_token(token: &str, auth0_authority: String) -> (bool, String, Vec<String>) {
    let jwks = fetch_jwks(&format!("{}{}", auth0_authority, ".well-known/jwks.json")).expect("failed to fetch jwks");
    let validations = vec![Validation::Issuer(auth0_authority), Validation::SubjectPresent];
    let kid = match token_kid(token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return (false, "".to_string(), vec![]),
    };

    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);

    let is_ok = res.is_ok();
    let res_unwraped = res.unwrap();
    println!("res_unwraped.headers = {:?}", res_unwraped.headers);
    println!("res_unwraped.claims = {:?}", res_unwraped.claims);

    let permissions = res_unwraped
        .claims
        .get("permissions")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.to_string().replace('\"', ""))
        .collect();

    (is_ok, res_unwraped.claims.get("sub").unwrap().to_string(), permissions)
}

fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri)?;
    let val = res.json::<JWKS>()?;
    Ok(val)
}
