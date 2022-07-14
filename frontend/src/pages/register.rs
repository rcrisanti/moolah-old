use reqwest::{Client, StatusCode};
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Header;
use crate::errors::InternalResponseError;
use crate::services::requests::fully_qualified_path;
use crate::{app::Route, components::AppContext};
use shared::{models::UserRegisterForm, routes};

pub enum RegisterMsg {
    AppContextUpdated(AppContext),
    UsernameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    PasswordConfirmChanged(String),
    Submitted,
    SuccessfulLogin(Route),
    Error(InternalResponseError),
}

pub struct Register {
    app_context: AppContext,
    username: String,
    email: String,
    password: String,
    password_confirm: String,
    redirect_to: Option<Route>,
    error_msg: Option<InternalResponseError>,
}

impl Component for Register {
    type Message = RegisterMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_context, _) = ctx
            .link()
            .context(ctx.link().callback(RegisterMsg::AppContextUpdated))
            .expect("no AppContext provided");

        Register {
            app_context,
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
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RegisterMsg::UsernameChanged(username) => {
                self.username = username;
                log::trace!("changed username");
            }
            RegisterMsg::EmailChanged(email) => {
                self.email = email;
                log::trace!("changed email");
            }
            RegisterMsg::PasswordChanged(password) => {
                self.password = password;
                log::trace!("changed password");
            }
            RegisterMsg::PasswordConfirmChanged(password_confirm) => {
                self.password_confirm = password_confirm;
                log::trace!("changed confirm password");
            }
            RegisterMsg::Submitted => {
                log::trace!("submitting form");
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
                                log::debug!("successfully submitted form");
                                RegisterMsg::SuccessfulLogin(Route::Home)
                            }
                            StatusCode::INTERNAL_SERVER_ERROR => RegisterMsg::Error(
                                InternalResponseError::Other("registration error".to_string()),
                            ),
                            _ => RegisterMsg::Error(InternalResponseError::Other(format!(
                                "unknown response status code {}",
                                response.status()
                            ))),
                        })
                        .emit(0);
                });
            }
            RegisterMsg::SuccessfulLogin(redirect_page) => {
                self.app_context.borrow_mut().login(self.username.clone());
                self.redirect_to = Some(redirect_page)
            }
            RegisterMsg::Error(err) => self.error_msg = Some(err),
            RegisterMsg::AppContextUpdated(context) => self.app_context = context,
        }
        true
    }
}
