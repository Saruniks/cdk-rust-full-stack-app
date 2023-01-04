use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    auth::auth_context::AuthContextProvider,
    components::{footer::Footer, nav::Nav},
    routes::{about::About, home::Home, pair_results_page::PairResultsPage, pair_voting_page::PairVotingPage, profile::Profile, AppRoute},
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <AuthContextProvider>
                <div class="flex flex-col h-screen">
                    <Nav/>
                    <main class="flex grow">
                        <div class="mx-auto py-6 sm:px-6 lg:px-8">
                            <Switch<AppRoute> render={Switch::render(switch)} />
                        </div>
                    </main>
                    <Footer />
                </div>
            </AuthContextProvider>
        </BrowserRouter>
    }
}

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },
        AppRoute::Profile => html! { <Profile /> },
        AppRoute::PairVoting => html! { <PairVotingPage /> },
        AppRoute::PairResults => html! { <PairResultsPage /> },
        AppRoute::About => html! { <About /> },
    }
}
