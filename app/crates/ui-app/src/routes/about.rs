use yew::prelude::*;

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        About {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <>
            <div class="w3-container" style="padding:128px 16px" id="about">
                <h3 class="w3-center">{"ABOUT US"}</h3>
                <p class="w3-center w3-large">{"Key features of Vendenic"}</p>
                <div class="w3-row-padding w3-center" style="margin-top:64px">
                    <div class="w3-half">
                        <i class="fas fa-users w3-margin-bottom w3-jumbo"></i>
                        <p class="w3-large">{"Community"}</p>
                        <p>{"Our aim is to build a community where different voices are welcomed, encouraged and respected."}</p>
                        <p>{"There is no place for hatred in Vendenic. If you notice anything - please report to us."}</p>
                    </div>
                    <div class="w3-half">
                        <i class="fa fa-poll w3-margin-bottom w3-jumbo w3-center"></i>
                        <p class="w3-large">{"Polls"}</p>
                        <p>{"We encourage everyone to express their thoughts by submitting a vote on a huge variety of polls."}</p>
                        <p>{"New polls and topics will appear soon."}</p>
                    </div>
                </div>
            </div>
        </>
        }
    }
}
