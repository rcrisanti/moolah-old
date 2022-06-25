use reqwest::{Client, StatusCode};
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use weblog::console_debug;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::{Footer, Header};
use crate::services::identity_remember;
use crate::services::requests::fully_qualified_path;
use shared::{models::UserRegisterForm, routes};

pub enum RegisterMsg {
    UsernameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    PasswordConfirmChanged(String),
    Submitted,
    SuccessfulLogin(Route),
    Error(RegisterError),
}

pub enum RegisterError {
    // Username(String),
    // Email(String),
    // Password(String),
    // PasswordConfirm(String),
    Other(String),
}

pub struct Register {
    username: String,
    email: String,
    password: String,
    password_confirm: String,
    redirect_to: Option<Route>,
    error_msg: Option<RegisterError>,
}

impl Component for Register {
    type Message = RegisterMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Register {
            username: String::new(),
            email: String::new(),
            password: String::new(),
            password_confirm: String::new(),
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
            input.map(|input| RegisterMsg::UsernameChanged(input.value()))
        });

        let onchange_email = ctx.link().batch_callback(|ev: Event| {
            let target = ev.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| RegisterMsg::EmailChanged(input.value()))
        });

        let onchange_password = ctx.link().batch_callback(|ev: Event| {
            let target = ev.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| RegisterMsg::PasswordChanged(input.value()))
        });

        let onchange_password_confirm = ctx.link().batch_callback(|ev: Event| {
            let target = ev.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| RegisterMsg::PasswordConfirmChanged(input.value()))
        });

        let onsubmit = ctx.link().callback(|ev: FocusEvent| {
            ev.prevent_default();
            RegisterMsg::Submitted
        });

        html! {
            <>
                <Header heading="register" title="register" />

                <form {onsubmit}>
                    <div>
                        <label for="username">{ "username:" }</label>
                        <input id="username" type="text" placeholder="username" onchange={onchange_username}/>
                    </div>
                    <div>
                        <label for="email">{ "email:" }</label>
                        <input id="email" type="email" placeholder="email" onchange={onchange_email}/>
                    </div>
                    <div>
                        <label for="password">{ "password:" }</label>
                        <input id="password" type="password" placeholder="password" onchange={onchange_password}/>
                    </div>
                    <div>
                        <label for="password-confirm">{ "confirm password:" }</label>
                        <input id="password-confirm" type="password" placeholder="password" onchange={onchange_password_confirm}/>
                    </div>
                    <input type="submit" value="register"/>
                </form>

                <Footer />
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RegisterMsg::UsernameChanged(username) => {
                self.username = username;
                console_debug!("changed username");
            }
            RegisterMsg::EmailChanged(email) => {
                self.email = email;
                console_debug!("changed email");
            }
            RegisterMsg::PasswordChanged(password) => {
                self.password = password;
                console_debug!("changed password");
            }
            RegisterMsg::PasswordConfirmChanged(password_confirm) => {
                self.password_confirm = password_confirm;
                console_debug!("changed confirm password");
            }
            RegisterMsg::Submitted => {
                console_debug!("submitting form");
                let user_form = UserRegisterForm::new(
                    self.username.clone(),
                    self.email.clone(),
                    self.password.clone(),
                    self.password_confirm.clone(),
                );

                let path = fully_qualified_path(routes::REGISTER.into())
                    .expect("could not build fully qualified path");

                let scope = Arc::new(ctx.link().clone());

                wasm_bindgen_futures::spawn_local(async move {
                    let response = Client::new()
                        .post(path)
                        .json(&user_form)
                        .send()
                        .await
                        .expect("could not post user form");

                    scope
                        .callback(move |_| match response.status() {
                            StatusCode::OK => {
                                console_debug!("successfully submitted form");
                                RegisterMsg::SuccessfulLogin(Route::Home)
                            }
                            StatusCode::INTERNAL_SERVER_ERROR => RegisterMsg::Error(
                                RegisterError::Other("registration error".to_string()),
                            ),
                            _ => RegisterMsg::Error(RegisterError::Other(format!(
                                "unknown response status code {}",
                                response.status()
                            ))),
                        })
                        .emit(0);
                });
            }
            RegisterMsg::SuccessfulLogin(redirect_page) => {
                identity_remember(self.username.clone().to_lowercase())
                    .expect("could not store identity in session storage");
                self.redirect_to = Some(redirect_page)
            }
            RegisterMsg::Error(err) => self.error_msg = Some(err),
        }
        true
    }
}
