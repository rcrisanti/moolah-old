use yew::prelude::*;

use crate::components::{Footer, Header};

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header heading="moolah" logged_in=false />

                <p>{ "this is the home page" }</p>

                <Footer />
            </>
        }
    }
}
