use yew::prelude::*;

pub struct Loading {}

impl Component for Loading {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Loading {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{ "loading..." }</div>
        }
    }
}
