use std::convert::TryFrom;

use auth0_spa_rust::{model::TokenOptions, GetTokenSilentlyOptions};
use gloo_timers::callback::Timeout;
use lazy_static::__Deref;
use openapi::apis::{configuration::Configuration, default_api::get_auth_info};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

use crate::{
    auth::{
        auth0_service::{API_URI, AUTH0_CLIENT_ID, AUTH0_DOMAIN, AUTH0_SERVICE},
        auth_context::AuthContext,
    },
    routes::AppRoute,
};

impl Default for AuthContext {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            loading: true,
            user: None,
            access_token: None,
        }
    }
}

pub fn use_session_logic() -> AuthContext {
    let use_history = use_history().expect("AnyHistory history is None. This error can happen if router was not provided.");

    let session_state_handle = use_state_eq(AuthContext::default);

    let session_state_handle_clone = session_state_handle.clone();
    use_effect(move || {
        Timeout::new(500, move || {
            spawn_local(async move {
                let auth_info = get_auth_info(&Configuration {
                    base_path: API_URI.get().expect("API_URI was not initialized").clone(),
                    user_agent: None,
                    client: reqwest::Client::new(),
                    basic_auth: None,
                    oauth_access_token: None,
                    api_key: None,
                    bearer_access_token: None,
                })
                .await
                .unwrap();

                log::debug!("Setting Auth0");

                AUTH0_DOMAIN.set(auth_info.auth0_domain).ok();
                AUTH0_CLIENT_ID.set(auth_info.auth0_client_id).ok();

                let is_authenticated = AUTH0_SERVICE.0.is_authenticated().await.as_bool().unwrap();
                let user_js = AUTH0_SERVICE.0.get_user(None).await;
                let user = match JsValue::into_serde(&user_js) {
                    Ok(user) => Some(user),
                    Err(_) => None,
                };

                let options = TokenOptions {
                    audience: "https://vendenic.com".to_string(),
                };

                let access_token = match AUTH0_SERVICE
                    .0
                    .get_token_silently(Some(
                        GetTokenSilentlyOptions::try_from(JsValue::from_serde(&options).unwrap()).unwrap(),
                    ))
                    .await
                {
                    Ok(token) => match JsValue::into_serde::<String>(&token) {
                        Ok(token) => Some(token),
                        Err(_err) => None,
                    },
                    Err(_) => None,
                };

                log::info!("handle_redirect_callback");
                // check window uri for login redirect data
                match web_sys::window() {
                    Some(window) => match window.location().search() {
                        Ok(path) => {
                            if path.contains("code=") {
                                spawn_local(async move {
                                    let _result = AUTH0_SERVICE.0.handle_redirect_callback(None).await;
                                    use_history.push(AppRoute::Home);
                                });
                            };
                        }
                        Err(err) => {
                            log::info!("window.location().search() error = {:?}", err);
                        }
                    },
                    None => {
                        log::info!("yew::web_sys::window() is None");
                    }
                };

                log::info!("is_authenticated = {}", is_authenticated);
                session_state_handle_clone.set(AuthContext {
                    is_authenticated,
                    user,
                    loading: false,
                    access_token,
                })
            });
        })
        .forget();
        || ()
    });

    session_state_handle.deref().clone()
}
