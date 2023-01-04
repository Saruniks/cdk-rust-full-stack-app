use yew_router::prelude::*;

pub mod about;
pub mod home;
pub mod pair_results_page;
pub mod pair_voting_page;
pub mod profile;

#[derive(Routable, Debug, Clone, Eq, PartialEq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[at("/profile")]
    Profile,
    #[at("/pair-voting")]
    PairVoting,
    #[at("/pair-results")]
    PairResults,
    #[at("/")]
    Home,
}
