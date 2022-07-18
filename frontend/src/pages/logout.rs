use reqwest::Client;
use shared::routes;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::AppContext;
use crate::requests::{fully_qualified_path, Requester, ResponseAction};
use crate::ResponseResult;

pub enum LogoutMsg {
    AppContextUpdated(AppContext),
    Logout,
    ReceivedResponse(ResponseResult<()>),
}

pub struct Logout {
    app_context: AppContext,
    response: Option<ResponseResult<()>>,
}

impl Component for Logout {
    type Message = LogoutMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_context, _) = ctx
            .link()
            .context(ctx.link().callback(LogoutMsg::AppContextUpdated))
            .expect("no AppContext provided");

        Logout {
            app_context,
            response: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.app_context.borrow_mut().is_logged_in() {
            ctx.link().callback(|_| LogoutMsg::Logout).emit(0);
        }

        html! {
            <Redirect<Route> to={Route::Home} />
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LogoutMsg::Logout => {
                self.app_context.borrow_mut().logout();

                let path = fully_qualified_path(routes::LOGOUT.into())
                    .expect("could not build fully qualified path");

                let scope = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let request = Client::new().post(path);
                    let on_ok = ResponseAction::from(|_| Ok(()));
                    let requester = Requester::default();
                    let response = requester.make(request, on_ok).await;

                    scope.send_message(LogoutMsg::ReceivedResponse(response));
                });
            }
            LogoutMsg::AppContextUpdated(context) => self.app_context = context,
            LogoutMsg::ReceivedResponse(response) => self.response = Some(response),
        }
        true
    }
}
