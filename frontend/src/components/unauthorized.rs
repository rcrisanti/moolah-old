use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

pub struct Unauthorized {}

impl Component for Unauthorized {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Unauthorized {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>{ "nice try. it seems you are not logged in, but you need to be to access this page." }</div>

                <div>
                    <Link<Route> to={Route::Login}>{ "login" }</Link<Route>>
                    <p>{ "or" }</p>
                    <Link<Route> to={Route::Register}>{ "register" }</Link<Route>>
                </div>
            </>
        }
    }
}
