use yew::{function_component, html, use_context};

use crate::auth::auth_context::AuthContext;

#[function_component(AuthInfoTestComponent)]
pub fn auth_info_test_component() -> Html {
    let auth_context = use_context::<AuthContext>().expect("no ctx found");

    html! {
        <div>
            <p>{format!("auth_context.is_authenticated = {}", auth_context.is_authenticated)}</p>
        </div>
    }
}
