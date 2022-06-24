use reqwest::{Client, StatusCode};
use shared::routes;
use weblog::console_error;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::services::requests::fully_qualified_path;
use crate::services::{identity_forget, identity_recall};

pub enum LogoutMsg {
    Logout,
}

pub struct Logout {}

impl Component for Logout {
    type Message = LogoutMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Logout {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if identity_recall().is_some() {
            ctx.link().callback(|_| LogoutMsg::Logout).emit(0);
        }

        html! {
            <Redirect<Route> to={Route::Home} />
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LogoutMsg::Logout => {
                identity_forget();

                let path = fully_qualified_path(routes::LOGOUT.into())
                    .expect("could not build fully qualified path");

                wasm_bindgen_futures::spawn_local(async move {
                    let response = Client::new()
                        .post(path)
                        .send()
                        .await
                        .expect("could not post logout");

                    if response.status() != StatusCode::OK {
                        console_error!("unable to log out");
                    }
                });
            }
        }
        true
    }
}
