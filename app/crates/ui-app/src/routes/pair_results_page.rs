use base64::decode;
use image::{EncodableLayout, ImageFormat};
use openapi::{
    apis::{
        configuration::Configuration,
        default_api::{get_voting_results, post_voting_results, GetMemberPairError, PostPairVoteError},
        Error,
    },
    models::{MemberPair, VotingResults},
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

use crate::auth::{auth0_service::API_URI, auth_context::AuthContext};

#[derive(PartialEq)]
pub struct PairResultsState {
    voting_results: VotingResults,
    finished_drawing: bool,
}

#[function_component(PairResultsPage)]
pub fn pair_results_page() -> Html {
    let state_handle = use_state_eq(|| PairResultsState {
        voting_results: VotingResults {
            results: vec![],
            total_pages: 0,
        },
        finished_drawing: false,
    });
    let current_page_handle = use_state_eq(|| 1);

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

    let access_token_clone = access_token.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                post_voting_results(&Configuration {
                    base_path: API_URI.get().expect("API_URI was not initialized").clone(),
                    user_agent: None,
                    client: reqwest::Client::new(),
                    basic_auth: None,
                    oauth_access_token: access_token_clone.clone(),
                    api_key: None,
                    bearer_access_token: None,
                })
                .await
                .unwrap();

                let response = get_voting_results(
                    &Configuration {
                        base_path: API_URI.get().expect("API_URI was not initialized").clone(),
                        user_agent: None,
                        client: reqwest::Client::new(),
                        basic_auth: None,
                        oauth_access_token: access_token_clone,
                        api_key: None,
                        bearer_access_token: None,
                    },
                    None,
                    Some(4),
                )
                .await;

                if let Ok(response) = response {
                    state_handle_clone.set(PairResultsState {
                        voting_results: response,
                        finished_drawing: false,
                    });
                };
            });
            || ()
        },
        (),
    );

    let state_handle_clone_0 = state_handle.clone();
    let state_handle_clone_1 = state_handle.clone();

    use_effect_with_deps(
        move |_| {
            log::info!("drddaw");
            spawn_local(async move {
                for (index, voting_result) in state_handle_clone_0.voting_results.results.iter().enumerate() {
                    draw_graph(index as i64, *voting_result.member_pair.clone()).await;
                    state_handle_clone_0.set(PairResultsState {
                        voting_results: state_handle_clone_0.voting_results.clone(),
                        finished_drawing: true,
                    });
                }
            });
            || ()
        },
        state_handle_clone_1.voting_results.clone(),
    );

    let drawings = state_handle
        .voting_results
        .results
        .iter()
        .enumerate()
        .map(|(index, result)| {
            html! {
                <div>
                    <canvas id={format!("0-{}-{}", index, result.member_pair.first_member.id.clone())} width="125" height="140">
                        {"Oh no! Your browser doesn't support canvas!"}
                    </canvas>
                    <canvas id={format!("1-{}-{}", index, result.member_pair.second_member.id.clone())} width="125" height="140">
                        {"Oh no! Your browser doesn't support canvas!"}
                    </canvas>
                    <p> {"DIFF: "} {result.diff_sum}</p>
                </div>
            }
        })
        .collect::<Html>();

    let state_handle_clone_1 = state_handle.clone();

    let current_page_handle_clone = current_page_handle.clone();
    let pagination_list = (1..state_handle.voting_results.total_pages)
        .into_iter()
        .map(|c| {
            let on_pagination_click = {
                let access_token_clone = access_token.clone();
                let state_handle_clone_2 = state_handle_clone_1.clone();
                let current_page_handle_clone = current_page_handle.clone();
                Callback::from(move |page: i32| {
                    let access_token_clone = access_token_clone.clone();
                    let state_handle_clone_3 = state_handle_clone_2.clone();
                    let current_page_handle_clone = current_page_handle_clone.clone();

                    state_handle_clone_3.set(PairResultsState {
                        voting_results: VotingResults {
                            results: vec![],
                            total_pages: 0,
                        },
                        finished_drawing: false,
                    });
                    current_page_handle_clone.set(page);

                    spawn_local(async move {
                        let response = get_voting_results(
                            &Configuration {
                                base_path: API_URI.get().expect("API_URI was not initialized").clone(),
                                user_agent: None,
                                client: reqwest::Client::new(),
                                basic_auth: None,
                                oauth_access_token: access_token_clone.clone(),
                                api_key: None,
                                bearer_access_token: None,
                            },
                            Some(page),
                            Some(4),
                        )
                        .await;

                        if let Ok(response) = response {
                            state_handle_clone_3.set(PairResultsState {
                                voting_results: response,
                                finished_drawing: false,
                            });
                        };
                        current_page_handle_clone.set(page);
                    })
                })
            };

            if c == *current_page_handle_clone as i64 {
                html! {
                    <li><a class="pagination-link is-current" aria-label={format!("Goto page {}", c)}
                        onclick={Callback::from(move |_| on_pagination_click.emit(c as i32))}>{c}</a></li>
                }
            } else {
                html! {
                    <li><a class="pagination-link" aria-label={format!("Goto page {}", c)}
                        onclick={Callback::from(move |_| on_pagination_click.emit(c as i32))}>{c}</a></li>
                }
            }
        })
        .collect::<Html>();

    html! {
        <>
            if !state_handle.finished_drawing {
                <svg role="status" class="w-8 h-8 mr-2 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                    <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
                </svg>
            }

            <div class="w3-center">
                {drawings}
                if state_handle.finished_drawing && state_handle.voting_results.total_pages > 1 {
                    <nav class="pagination" role="navigation" aria-label="pagination">
                        <ul class="flex items-center justify-center pagination-list">
                            {pagination_list}
                        </ul>
                    </nav>
                }
            </div>
        </>
    }
}

// impl PairResultsPage {
async fn draw_graph(index: i64, member_pair: MemberPair) {
    let decoded_bytes = decode(member_pair.clone().first_member.file).unwrap();

    let image = image::load_from_memory_with_format(decoded_bytes.as_bytes(), ImageFormat::WebP).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(&format!("0-{}-{}", index, member_pair.first_member.id.clone()))
        .unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let image: ImageData = ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(&image.to_rgba8()), 125).unwrap();

    let bitmap_promise = web_sys::window().unwrap().create_image_bitmap_with_image_data(&image).unwrap();
    let bitmap = wasm_bindgen_futures::JsFuture::from(bitmap_promise)
        .await
        .unwrap()
        .dyn_into::<web_sys::ImageBitmap>()
        .unwrap();

    context.draw_image_with_image_bitmap(&bitmap, 0.0, 0.0).unwrap();

    // 2nd item
    let decoded_bytes = decode(member_pair.clone().second_member.file).unwrap();

    let image = image::load_from_memory_with_format(decoded_bytes.as_bytes(), ImageFormat::WebP).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(&format!("1-{}-{}", index, member_pair.second_member.id.clone()))
        .unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let image: ImageData = ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(&image.to_rgba8()), 125).unwrap();

    let bitmap_promise = web_sys::window().unwrap().create_image_bitmap_with_image_data(&image).unwrap();
    let bitmap = wasm_bindgen_futures::JsFuture::from(bitmap_promise)
        .await
        .unwrap()
        .dyn_into::<web_sys::ImageBitmap>()
        .unwrap();

    context.draw_image_with_image_bitmap(&bitmap, 0.0, 0.0).unwrap();
}
