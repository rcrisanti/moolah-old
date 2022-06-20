use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or(None)]
    pub title: Option<String>,
    pub heading: String,
    pub logged_in: bool,
}

pub struct Header {}

impl Component for Header {
    type Message = ();
    type Properties = HeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Header {}
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
                        {
                            if ctx.props().logged_in {
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
}