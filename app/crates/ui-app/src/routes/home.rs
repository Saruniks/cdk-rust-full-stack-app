use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="rounded-lg bg-gray-200 shadow-lg px-5 pt-5 pb-10 text-gray-800">
                <div class="w-full pt-1 pb-5">
                </div>
                <div class="w-full mb-10">
                    <div class="text-3xl text-gray-600 text-left leading-tight h-3">{"“"}</div>
                    <p class="text-sm text-gray-800 text-center px-5"><i>{"Lorem ipsum dolor sit amet,
                    consectetur adipiscing elit, sed do eiusmod tempor 
                    incididunt ut labore et dolore magna aliqua. Ut enim 
                    ad minim veniam, quis nostrud exercitation ullamco laboris 
                    nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor 
                    in reprehenderit in voluptate velit esse cillum dolore eu fugiat 
                    nulla pariatur. Excepteur sint occaecat cupidatat non proident, 
                    sunt in culpa qui officia deserunt mollit anim id est laborum."}</i></p>
                    <div class="text-3xl text-gray-600 text-right leading-tight h-3 -mt-3">{"”"}</div>
                </div>
                // <div class="w-full">
                //     <p class="text-md text-gray-600 font-bold text-center"><i>{"Me"}</i></p>
                // </div>
            </div>
        }
    }
}
