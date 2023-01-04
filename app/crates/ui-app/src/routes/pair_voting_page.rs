use base64::decode;
use image::{EncodableLayout, ImageFormat};
use openapi::{
    apis::{
        configuration::Configuration,
        default_api::{get_member_pair, post_pair_vote, GetMemberPairError, PostPairVoteError},
        Error,
    },
    models::{MemberPair, PostPairVoteRequest},
};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::ImageData;
use yew::prelude::*;

pub enum Msg {
    GetItemsResponse(Result<MemberPair, Error<GetMemberPairError>>),
    PostPairVoteResponse(Result<(), Error<PostPairVoteError>>),
    Update,
    Submit(i64),
}
use material_yew::MatButton;

use crate::auth::{auth0_service::API_URI, auth_context::AuthContext};

#[derive(PartialEq)]
pub struct PairVotingState {
    first_canvas_id: String,
    second_canvas_id: String,
    items: Option<MemberPair>,
}

#[function_component(PairVotingPage)]
pub fn pair_voting_page() -> Html {
    let state_handle = use_state_eq(|| PairVotingState {
        first_canvas_id: "canvas-first".to_string(),
        second_canvas_id: "canvas-second".to_string(),
        items: None,
    });

    let state_handle_clone = state_handle.clone();

    let AuthContext { loading, access_token, .. } = use_context::<AuthContext>().expect("no ctx found");
    if loading {
        return html! {
            <svg role="status" class="w-8 h-8 mr-2 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
            </svg>
        };
    }

    if access_token.is_none() {
        return html! { <div>{"Login Required"}</div> };
    }

    {
        let access_token = access_token.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let response = get_member_pair(&Configuration {
                        base_path: API_URI.get().expect("API_URI was not initialized").clone(),
                        user_agent: None,
                        client: reqwest::Client::new(),
                        basic_auth: None,
                        oauth_access_token: access_token.clone(),
                        api_key: None,
                        bearer_access_token: None,
                    })
                    .await;

                    if let Ok(response) = response {
                        state_handle_clone.set(PairVotingState {
                            first_canvas_id: "canvas-first".to_string(),
                            second_canvas_id: "canvas-second".to_string(),
                            items: Some(response),
                        });
                    };
                });
                || ()
            },
            (),
        );
    }

    let state_handle_clone_0 = state_handle.clone();
    let state_handle_clone_1 = state_handle.clone();

    use_effect_with_deps(
        move |_| {
            log::info!("drddaw");
            if let Some(items) = state_handle_clone_0.items.clone() {
                draw_graph(items);
            }
            || ()
        },
        state_handle_clone_1.items.clone(),
    );

    let state_handle_clone = state_handle.clone();

    let onclick = if state_handle.items.is_some() {
        Callback::from(move |vote: i64| {
            log::info!("vote = {}", vote);
            let token = access_token.clone();
            let state_handle_clone = state_handle_clone.clone();
            let first_member_id = state_handle_clone.items.clone().unwrap().first_member.id;
            let second_member_id = state_handle_clone.items.clone().unwrap().second_member.id;

            spawn_local(async move {
                let config = Configuration {
                    base_path: API_URI.get().expect("API_URI was not initialized").clone(),
                    user_agent: None,
                    client: reqwest::Client::new(),
                    basic_auth: None,
                    oauth_access_token: token.clone(),
                    api_key: None,
                    bearer_access_token: None,
                };
                let response = post_pair_vote(
                    &config,
                    PostPairVoteRequest {
                        first_member_id,
                        second_member_id,
                        diff: vote,
                    },
                );

                state_handle_clone.set(PairVotingState {
                    first_canvas_id: "canvas-first".to_string(),
                    second_canvas_id: "canvas-second".to_string(),
                    items: None,
                });

                response.await.unwrap();

                let response = get_member_pair(&Configuration {
                    base_path: API_URI.get().expect("API_URI was not initialized").clone(),
                    user_agent: None,
                    client: reqwest::Client::new(),
                    basic_auth: None,
                    oauth_access_token: token,
                    api_key: None,
                    bearer_access_token: None,
                })
                .await;

                if let Ok(response) = response {
                    state_handle_clone.set(PairVotingState {
                        first_canvas_id: "canvas-first".to_string(),
                        second_canvas_id: "canvas-second".to_string(),
                        items: Some(response),
                    });
                };
            });
        })
    } else {
        Callback::from(move |_vote: i64| {})
    };

    let onclick_minus_two = onclick.clone();
    let onclick_minus_one = onclick.clone();
    let onclick_equal = onclick.clone();
    let onclick_plus_one = onclick.clone();
    let onclick_plus_two = onclick;

    html! {
        <>
            if let Some(_items) = &state_handle.items {
                <div class="w3-center">
                    <div>
                        <canvas id={state_handle.first_canvas_id.clone()} width="125" height="140">
                            {"Oh no! Your browser doesn't support canvas!"}
                        </canvas>
                        <canvas id={state_handle.second_canvas_id.clone()} width="125" height="140">
                            {"Oh no! Your browser doesn't support canvas!"}
                        </canvas>
                    </div>
                    <div>
                        <span onclick={Callback::from(move |_| onclick_minus_two.emit(-2))}>
                            <MatButton label="+2" />
                        </span>
                        <span onclick={Callback::from(move |_| onclick_minus_one.emit(-1))}>
                            <MatButton label="+1" />
                        </span>
                        <span onclick={Callback::from(move |_| onclick_equal.emit(0))}>
                            <MatButton label="0" />
                        </span>
                        <span onclick={Callback::from(move |_| onclick_plus_one.emit(1))}>
                            <MatButton label="+1" />
                        </span>
                        <span onclick={Callback::from(move |_| onclick_plus_two.emit(2))}>
                            <MatButton label="+2" />
                        </span>
                        </div>
                </div>
            } else {
                <svg role="status" class="w-8 h-8 mr-2 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                    <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
                </svg>
            }
        </>
    }
}

// impl PairVotingPage {
fn draw_graph(items: MemberPair) {
    let decoded_bytes = decode(items.clone().first_member.file).unwrap();

    let image = image::load_from_memory_with_format(decoded_bytes.as_bytes(), ImageFormat::WebP).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas-first").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let image: ImageData = ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(&image.to_rgba8()), 125).unwrap();

    context.put_image_data(&image, 0.0, 0.0).unwrap();

    // 2nd item
    let decoded_bytes = decode(items.second_member.file).unwrap();

    let image = image::load_from_memory_with_format(decoded_bytes.as_bytes(), ImageFormat::WebP).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas-second").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let image: ImageData = ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(&image.to_rgba8()), 125).unwrap();

    context.put_image_data(&image, 0.0, 0.0).unwrap();
}
