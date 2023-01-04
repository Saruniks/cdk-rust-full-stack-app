use std::convert::TryFrom;

use auth0_spa_rust::{model::AuthLogoutOptions, LogoutOptions};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    auth::{
        auth0_service::{AUTH0_REDIRECT_URI, AUTH0_SERVICE},
        auth_context::AuthContext,
    },
    routes::AppRoute,
};

#[function_component(Nav)]
pub fn nav() -> Html {
    let on_login = Callback::from(move |_| {
        spawn_local(async move {
            AUTH0_SERVICE.0.login_with_redirect(None).await;
        })
    });
    let on_login_clone = on_login.clone();
    let on_logout = Callback::from(move |_| {
        spawn_local(async move {
            let logout_options = AuthLogoutOptions {
                returnTo: AUTH0_REDIRECT_URI.get().expect("AUTH0_REDIRECT_URI not set").to_string(),
            };

            AUTH0_SERVICE.0.logout(Some(
                LogoutOptions::try_from(JsValue::from_serde(&logout_options).unwrap()).unwrap(),
            ));
        })
    });
    let on_logout_clone = on_logout.clone();

    let show_menu_handle = use_state_eq(|| false);
    let show_menu_handle_clone_1 = show_menu_handle.clone();
    let show_menu_handle_clone_2 = show_menu_handle.clone();
    let show_menu_handle_clone_3 = show_menu_handle.clone();
    let show_menu_handle_clone_4 = show_menu_handle.clone();
    let show_menu_handle_clone_5 = show_menu_handle.clone();
    let show_menu_handle_clone_6 = show_menu_handle.clone();
    let show_menu_handle_clone_7 = show_menu_handle.clone();

    let show_menu_handle_clone = show_menu_handle.clone();
    let on_menu_btn_click = Callback::from(move |_| {
        show_menu_handle_clone.set(!*show_menu_handle_clone);
    });

    let auth_context = use_context::<AuthContext>().expect("no ctx found");
    let use_history = use_history().expect("AnyHistory history is None. This error can happen if router was not provided.");
    let use_history_clone_0 = use_history.clone();
    let use_history_clone_1 = use_history.clone();
    let use_history_clone_2 = use_history.clone();
    let use_history_clone_3 = use_history.clone();
    let use_history_clone_4 = use_history.clone();
    let use_history_clone_5 = use_history.clone();
    html! {
        <nav class="relative bg-gray-800">
            <div class="max-w-7xl mx-auto px-8">
                <div class="flex space-x-4 items-center justify-between h-16 text-white">
                    <a onclick={Callback::from(move |_| {
                        show_menu_handle_clone_4.set(false);
                        use_history.push(AppRoute::Home)
                    })}
                        class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium w3-wide">{"VENDENIC"}</a>
                    // Big screen
                    <div class="hidden md:flex space-x-4 items-center justify-between h-16 text-white">
                        <div class="flex justify-start space-x-4">
                            <a onclick={Callback::from(move |_| {
                                show_menu_handle_clone_5.set(false);
                                use_history_clone_2.push(AppRoute::PairResults)
                            })}
                                class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">{"Results"}</a>
                            <a onclick={Callback::from(move |_| {
                                show_menu_handle_clone_6.set(false);
                                use_history_clone_0.push(AppRoute::PairVoting)
                            })}
                                class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">{"Voting"}</a>
                        </div>
                        <div class="flex justify-end">
                                {
                                    match auth_context.is_authenticated {
                                        true => {
                                            html! {
                                                <div class="flex space-x-4">
                                                    <a onclick={Callback::from(move |_| {
                                                        show_menu_handle_clone_7.set(false);
                                                        use_history_clone_1.push(AppRoute::Profile)
                                                    })}
                                                        class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"><i class="fa fa-user"></i>{" "}{auth_context.user.clone().unwrap().given_name}</a>
                                                    <a onclick={on_logout}
                                                        class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"> {"Sign Out"}</a>
                                                </div>
                                            }
                                        }
                                        false => {
                                            html! {
                                                <div class="flex space-x-0">
                                                    <a onclick={on_login}
                                                        class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">{"Login"}</a>
                                                    <div class="flex flex-shrink-0 items-center">
                                                        <img class="h-6 w-6" src="https://cdn.auth0.com/styleguide/components/1.0.8/media/logos/img/badge.png" alt="Workflow"/> // maybe host png myself
                                                    </div>
                                                </div>
                                            }
                                        }
                                    }
                                }
                        </div>
                    </div>
                    // Small screens
                    if *show_menu_handle {
                        <a id="menu-btn" onclick={on_menu_btn_click} class="bg-gray-800 md:hidden" >
                            <div class="space-y-2">
                                <div class="w-7 h-0.5 bg-gray-300"></div>
                                <div class="w-7 h-0.5 bg-gray-300"></div>
                                <div class="w-7 h-0.5 bg-gray-300"></div>
                            </div>
                        </a>
                    }
                </div>
            </div>
            // Mobile Menu
            <div class="md:hidden">
                if *show_menu_handle {
                    <div id="menu" class="bg-gray-800 absolute flex flex-col items-center self-end py-8 mt-10
                        space-y-6 font-bold sm:w-auto sm:self-center left-6 right-6 drop-shadow md z-40">
                        <a onclick={Callback::from(move |_| {
                            show_menu_handle_clone_1.set(false);
                            use_history_clone_3.push(AppRoute::PairResults)
                        })}
                            class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">{"Results"}</a>
                        <a onclick={Callback::from(move |_| {
                            show_menu_handle_clone_2.set(false);
                            use_history_clone_4.push(AppRoute::PairVoting)
                        })}
                            class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">{"Voting"}</a>
                            {
                                match auth_context.is_authenticated {
                                    true => {
                                        html! {
                                            <>
                                                <a onclick={Callback::from(move |_| {
                                                    show_menu_handle_clone_3.set(false);
                                                    use_history_clone_5.push(AppRoute::Profile)
                                                })}
                                                    class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"><i class="fa fa-user"></i>{" "}{auth_context.user.unwrap().given_name}</a>
                                                <a onclick={on_logout_clone}
                                                    class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium"> {"Sign Out"}</a>
                                            </>
                                        }
                                    }
                                    false => {
                                        html! {
                                            <div class="flex space-x-0">
                                                <a onclick={on_login_clone}
                                                    class="text-gray-300 hover:bg-gray-700 hover:text-white px-3 py-2 rounded-md text-sm font-medium">{"Login"}</a>
                                                <div class="flex flex-shrink-0 items-center">
                                                    <img class="h-6 w-6" src="https://cdn.auth0.com/styleguide/components/1.0.8/media/logos/img/badge.png" alt="Workflow"/> // maybe host png myself
                                                </div>
                                            </div>
                                        }
                                    }
                                }
                            }
                    </div>
                }
            </div>
            </nav>
    }
}
