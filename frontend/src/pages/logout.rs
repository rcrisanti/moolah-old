use reqwest::{Client, StatusCode};
use shared::routes;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::AppContext;
use crate::services::requests::fully_qualified_path;

pub enum LogoutMsg {
    AppContextUpdated(AppContext),
    Logout,
}

pub struct Logout {
    app_context: AppContext,
}

impl Component for Logout {
    type Message = LogoutMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_context, _) = ctx
            .link()
            .context(ctx.link().callback(LogoutMsg::AppContextUpdated))
            .expect("no AppContext provided");

        Logout { app_context }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.app_context.borrow().is_logged_in() {
            ctx.link().callback(|_| LogoutMsg::Logout).emit(0);
        }

        html! {
            <Redirect<Route> to={Route::Home} />
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LogoutMsg::Logout => {
                self.app_context.borrow_mut().logout();

                let path = fully_qualified_path(routes::LOGOUT.into())
                    .expect("could not build fully qualified path");

                wasm_bindgen_futures::spawn_local(async move {
                    let response = Client::new()
                        .post(path)
                        .send()
                        .await
                        .expect("could not post logout");

                    if response.status() != StatusCode::OK {
                        log::error!("unable to log out");
                    }
                });
            }
            LogoutMsg::AppContextUpdated(context) => self.app_context = context,
        }
        true
    }
}
