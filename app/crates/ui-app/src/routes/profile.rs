use yew::prelude::*;

use crate::auth::auth_context::AuthContext;

#[function_component(Profile)]
pub fn profile() -> Html {
    let auth_context = use_context::<AuthContext>().expect("no ctx found");

    match auth_context.user {
        Some(user) => {
            html! {
                <div class="w3-container" style="padding:128px 16px" id="profile">
                    <h3 class="w3-center">{"PROFILE"}</h3>
                    <p class="w3-center w3-large">
                        {"Name: "}{&user.given_name}
                        <br/>
                        {"Surname: "}{&user.family_name}
                    </p>
                </div>
            }
        }
        None => {
            html! {
                <p class="w3-center w3-large">
                    {"Failed to load profile"}
                </p>
            }
        }
    }
}
