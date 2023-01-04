use auth0_spa_rust::model::User;
use yew::prelude::*;

use crate::auth::session_logic::use_session_logic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthContext {
    pub is_authenticated: bool,
    pub user: Option<User>,
    pub loading: bool,
    pub access_token: Option<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AuthContextProvider)]
pub fn auth_context(props: &Props) -> Html {
    // Other way to get history, so get higher than browser

    let context = use_session_logic();

    let context_clone = context.clone();
    use_effect_with_deps(
        move |_| {
            if !context_clone.loading {}
            || ()
        },
        context.clone(),
    );

    html! {
        <ContextProvider<AuthContext> {context}>
            {props.children.clone()}
        </ContextProvider<AuthContext>>
    }
}
