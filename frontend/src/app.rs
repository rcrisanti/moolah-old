use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home, Register};

#[derive(Routable, PartialEq, Clone, Copy, Debug)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/login")]
    Login,

    #[at("/logout")]
    Logout,

    #[at("/account")]
    Account,

    #[at("/register")]
    Register,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Login => html! {"login"},
        Route::NotFound => html! {"404"},
        Route::Logout => html! {"logout"},
        Route::Account => html! {"account"},
        Route::Register => html! { <Register /> },
    }
}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}
