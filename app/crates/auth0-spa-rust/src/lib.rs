pub mod model;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    pub type Auth0Client;

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = Auth0ClientOptions)]
    pub type Auth0ClientOptions;
    #[wasm_bindgen(constructor)]
    pub fn new(options: Auth0ClientOptions) -> Auth0Client;

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = CacheLocation)]
    pub type CacheLocation;
    #[wasm_bindgen(method, getter = cacheLocation)]
    pub fn cache_location(this: &Auth0Client) -> CacheLocation;
    #[wasm_bindgen(method, setter = cacheLocation)]
    pub fn set_cache_location(this: &Auth0Client, value: CacheLocation);

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = RedirectLoginOptions)]
    pub type RedirectLoginOptions;
    #[wasm_bindgen(method, js_name = buildAuthorizeUrl)] // Return String
    pub async fn build_authorize_url(this: &Auth0Client, options: Option<RedirectLoginOptions>) -> JsValue;

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = LogoutUrlOptions)]
    pub type LogoutUrlOptions;
    #[wasm_bindgen(method, js_name = buildLogoutUrl)]
    pub fn build_logout_url(this: &Auth0Client, options: LogoutUrlOptions) -> String;

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = GetTokenSilentlyOptions)]
    pub type GetTokenSilentlyOptions;
    #[wasm_bindgen(method, js_name = CheckSession)]
    pub async fn check_session(this: &Auth0Client, options: Option<GetTokenSilentlyOptions>);

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = GetIdTokenClaimsOptions)]
    pub type GetIdTokenClaimsOptions;
    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = IdToken)]
    pub type IdToken;
    #[wasm_bindgen(method, js_name = getIdTokenClaims)] // Return IdToken
    pub async fn get_id_token_claims(this: &Auth0Client, options: Option<GetIdTokenClaimsOptions>) -> JsValue;

    #[wasm_bindgen(method, catch, js_name = getTokenSilently)]
    pub async fn get_token_silently(this: &Auth0Client, options: Option<GetTokenSilentlyOptions>) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = GetTokenWithPopupOptions)]
    pub type GetTokenWithPopupOptions;
    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = PopupConfigOptions)]
    pub type PopupConfigOptions;
    #[wasm_bindgen(method, js_name = getTokenWithPopup)] // Return String
    pub async fn get_token_with_popup(
        this: &Auth0Client,
        options: Option<GetTokenWithPopupOptions>,
        config: Option<PopupConfigOptions>,
    ) -> JsValue;

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = GetUserOptions)]
    pub type GetUserOptions;
    #[wasm_bindgen(method, js_name = getUser)]
    pub async fn get_user(this: &Auth0Client, options: Option<GetUserOptions>) -> JsValue;

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = RedirectLoginResult)]
    pub type RedirectLoginResult;
    #[wasm_bindgen(method, catch, js_name = handleRedirectCallback)] // Return RedirectLoginResult
    pub async fn handle_redirect_callback(this: &Auth0Client, url: Option<String>) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = isAuthenticated)] // Return bool
    pub async fn is_authenticated(this: &Auth0Client) -> JsValue;

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = PopupLoginOptions)]
    pub type PopupLoginOptions;
    #[wasm_bindgen(method, js_name = loginWithPopup)]
    pub async fn login_with_popup(this: &Auth0Client, options: Option<PopupLoginOptions>, config: Option<PopupConfigOptions>);

    #[wasm_bindgen(method, js_name = loginWithRedirect)]
    pub async fn login_with_redirect(this: &Auth0Client, options: Option<RedirectLoginOptions>);

    #[wasm_bindgen(extends = :: js_sys :: Object , js_name = LogoutOptions)]
    pub type LogoutOptions;
    #[wasm_bindgen(method)]
    pub fn logout(this: &Auth0Client, options: Option<LogoutOptions>);
}

unsafe impl Sync for Auth0Client {}
unsafe impl Send for Auth0Client {}
