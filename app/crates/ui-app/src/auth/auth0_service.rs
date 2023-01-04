use std::convert::TryFrom;

use auth0_spa_rust::{model::ConfigOptions, Auth0Client, Auth0ClientOptions};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use wasm_bindgen::JsValue;

pub static AUTH0_DOMAIN: OnceCell<String> = OnceCell::new();
pub static AUTH0_CLIENT_ID: OnceCell<String> = OnceCell::new();
pub static AUTH0_REDIRECT_URI: OnceCell<String> = OnceCell::new();
pub static AUTH0_USE_REFRESH_TOKENS: OnceCell<bool> = OnceCell::new();
pub static AUTH0_CACHE_LOCATION: OnceCell<String> = OnceCell::new();
pub static API_URI: OnceCell<String> = OnceCell::new();

lazy_static! {
    pub static ref AUTH0_SERVICE: Auth0Service = Auth0Service::new();
}

pub struct Auth0Service(pub Auth0Client);

impl Auth0Service {
    pub fn new() -> Self {
        let options = ConfigOptions {
            domain: AUTH0_DOMAIN.get().expect("AUTH0_DOMAIN not set").to_string(),
            client_id: AUTH0_CLIENT_ID.get().expect("AUTH0_CLIENT_ID not set").to_string(),
            redirect_uri: AUTH0_REDIRECT_URI.get().expect("AUTH0_REDIRECT_URI not set").to_string(),
            useRefreshTokens: *AUTH0_USE_REFRESH_TOKENS.get().expect("AUTH0_USE_REFRESH_TOKENS not set"),
            cacheLocation: AUTH0_CACHE_LOCATION.get().expect("AUTH0_CACHE_LOCATION not set").to_string(),
            audience: "https://vendenic.com".to_string(),
        };

        Auth0Service(Auth0Client::new(
            Auth0ClientOptions::try_from(JsValue::from_serde(&options).unwrap()).unwrap(),
        ))
    }
}

impl Default for Auth0Service {
    fn default() -> Self {
        Self::new()
    }
}
