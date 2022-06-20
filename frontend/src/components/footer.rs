use yew::prelude::*;

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Footer {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer>
                <hr/>
                <p>{ "\u{A9} 2022 Ryan Crisanti" } </p> // TODO: make date dynamic
                <p>{ "made with "} <a href="https://yew.rs/">{"Yew"}</a></p>
            </footer>
        }
    }
}
