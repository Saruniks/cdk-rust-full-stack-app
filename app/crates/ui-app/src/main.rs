#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::mixed_read_write_in_expression)]
#![allow(clippy::let_unit_value)]

pub mod app;
pub mod auth;
pub mod components;
pub mod routes;

use app::App;
use auth::auth0_service::{API_URI, AUTH0_CACHE_LOCATION, AUTH0_REDIRECT_URI, AUTH0_USE_REFRESH_TOKENS};
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    let origin = web_sys::window()
        .expect("Unable to get window")
        .location()
        .origin()
        .expect("Unable to get origin");
    AUTH0_REDIRECT_URI.set(origin.clone()).expect("Couldn't set AUTH0_REDIRECT_URI");
    AUTH0_USE_REFRESH_TOKENS.set(false).expect("Couldn't set AUTH0_USE_REFRESH_TOKENS");
    AUTH0_CACHE_LOCATION
        .set("localstorage".to_string())
        .expect("Couldn't set AUTH0_CACHE_LOCATION");

    API_URI.set(origin).expect("Couldn't set API_URI");

    yew::start_app::<App>();
    Ok(())
}
