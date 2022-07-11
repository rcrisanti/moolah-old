use std::fmt::Display;
use std::sync::Arc;

use reqwest::{Client, StatusCode};
use shared::models::{User, UserLoginRequestForm};
use shared::routes;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::Header;
use crate::services::identity_remember;
use crate::services::requests::fully_qualified_path;

pub enum LoginMsg {
    UsernameChanged(String),
    PasswordChanged(String),
    Submitted,
    SuccessfulLogin(Route),
    Error(LoginError),
}

#[derive(Clone)]
pub enum LoginError {
    IncorrectCredentials,
    Other(String),
}

impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncorrectCredentials => write!(f, "incorrect credentials"),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}

pub struct Login {
    username: String,
    password: String,
    redirect_to: Option<Route>,
    error_msg: Option<LoginError>,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Login {
            username: String::new(),
            password: String::new(),
            redirect_to: None,
            error_msg: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(redirect_to) = &self.redirect_to {
            return html! {
                <Redirect<Route> to={redirect_to.clone()} />
            };
        }

        let onchange_username = ctx.link().batch_callback(|ev: Event| {
            let target = ev.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| LoginMsg::UsernameChanged(input.value()))
        });

        let onchange_password = ctx.link().batch_callback(|ev: Event| {
            let target = ev.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| LoginMsg::PasswordChanged(input.value()))
        });

        let onsubmit = ctx.link().callback(|ev: FocusEvent| {
            ev.prevent_default();
            LoginMsg::Submitted
        });

        html! {
            <>
                <Header heading="login" title="login" />

                <form {onsubmit}>
                    {
                        if let Some(error_msg) = self.error_msg.clone() {
                            html! {
                                <p>{ format!("error: {}", error_msg) }</p>
                            }
                        } else {
                            html! { <></> }
                        }
                    }
                    <div>
                        <label for="username">{ "username:" }</label>
                        <input id="username" type="text" placeholder="username" onchange={onchange_username}/>
                    </div>
                    <div>
                        <label for="password">{ "password:" }</label>
                        <input id="password" type="password" placeholder="password" onchange={onchange_password}/>
                    </div>
                    <input type="submit" value="login"/>
                </form>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::UsernameChanged(username) => {
                self.username = username;
                log::trace!("changed username");
            }
            LoginMsg::PasswordChanged(password) => {
                self.password = password;
                log::trace!("changed password");
            }
            LoginMsg::Submitted => {
                log::trace!("submitting form");
                let user_form = UserLoginRequestForm::new(self.username.clone());

                let path = fully_qualified_path(routes::LOGIN_REQUEST_PASSWORD.into())
                    .expect("could not build fully qualified path");

                let scope = Arc::new(ctx.link().clone());
                let password = self.password.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Client::new();
                    let response = client
                        .post(path)
                        .json(&user_form)
                        .send()
                        .await
                        .expect("could not post user form");

                    match response.status() {
                        StatusCode::OK => {
                            let user: User = response
                                .json()
                                .await
                                .expect("could not retrieve user from response");

                            if user
                                .verify_user(user_form.username, password)
                                .expect("could not verify user")
                            {
                                let response = client
                                    .post(fully_qualified_path(routes::LOGIN.into()).expect(
                                        "could not build fully qualified path for second request",
                                    ))
                                    .json(&user)
                                    .send()
                                    .await
                                    .expect("could not post user");

                                if response.status() == StatusCode::OK {
                                    scope
                                        .callback(|_| LoginMsg::SuccessfulLogin(Route::Home))
                                        .emit(0);
                                } else {
                                    scope
                                        .callback(|_| {
                                            LoginMsg::Error(LoginError::IncorrectCredentials)
                                        })
                                        .emit(0);
                                }
                            } else {
                                scope
                                    .callback(|_| LoginMsg::Error(LoginError::IncorrectCredentials))
                                    .emit(0);
                            }
                        }
                        StatusCode::INTERNAL_SERVER_ERROR => {
                            scope
                                .callback(|_| {
                                    LoginMsg::Error(LoginError::Other(
                                        "username does not exist".to_string(),
                                    ))
                                })
                                .emit(0);
                        }
                        _ => {
                            scope
                                .callback(move |_| {
                                    LoginMsg::Error(LoginError::Other(format!(
                                        "unknown response status code {}",
                                        response.status()
                                    )))
                                })
                                .emit(0);
                        }
                    };
                });
            }
            LoginMsg::SuccessfulLogin(redirect_page) => {
                identity_remember(self.username.clone().to_lowercase())
                    .expect("could not store identity in session storage");
                self.redirect_to = Some(redirect_page)
            }
            LoginMsg::Error(err) => self.error_msg = Some(err),
        }
        true
    }
}
