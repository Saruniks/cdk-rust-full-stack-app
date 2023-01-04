use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Footer {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class="w3-center text-gray-300 bg-gray-800 w3-padding-32">
                <p>{"Vendenic team: info@vendenic.com"}</p>
                <p>{"v0.2.1 © 2021"}</p>
            </footer>
        }
    }
}
