use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::AppContext;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or(None)]
    pub title: Option<String>,
    pub heading: String,
}

pub enum HeaderMsg {
    AppContextUpdated(AppContext),
}

pub struct Header {
    app_context: AppContext,
}

impl Component for Header {
    type Message = HeaderMsg;
    type Properties = HeaderProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_context, _) = ctx
            .link()
            .context(ctx.link().callback(HeaderMsg::AppContextUpdated))
            .expect("no AppContext provided");

        Header { app_context }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = match &ctx.props().title {
            Some(title) => format!("moolah | {}", title),
            None => "moolah".to_string(),
        };

        html! {
            <>
                <head>
                    <title>{ title }</title>
                </head>
                <header>
                    <h1>{ &ctx.props().heading }</h1>

                    <div style={"float:right;"}>
                        <Link<Route> to={Route::Home}>{ "home" }</Link<Route>>
                        {
                            if self.app_context.borrow_mut().is_logged_in() {
                                html! {
                                    <>
                                        <Link<Route> to={Route::Account}>{ "account" }</Link<Route>>
                                        <Link<Route> to={Route::Logout}>{ "logout" }</Link<Route>>
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                        <Link<Route> to={Route::Login}>{ "login" }</Link<Route>>
                                        <Link<Route> to={Route::Register}>{ "register" }</Link<Route>>
                                    </>
                                }
                            }
                        }
                    </div>
                    <hr/>
                </header>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HeaderMsg::AppContextUpdated(context) => {
                if context.borrow_mut().is_logged_in()
                    == self.app_context.borrow_mut().is_logged_in()
                {
                    return false;
                }
            }
        }
        true
    }
}
