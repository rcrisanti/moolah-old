use reqwest::Client;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Header;
use crate::requests::{fully_qualified_path, Requester, ResponseAction};
use crate::ResponseResult;
use crate::{app::Route, components::AppContext};
use shared::{models::UserRegisterForm, routes};

pub enum RegisterMsg {
    AppContextUpdated(AppContext),
    UsernameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    PasswordConfirmChanged(String),
    Submitted,
    ResponseReceived(ResponseResult<Route>),
}

pub struct Register {
    app_context: AppContext,
    username: String,
    email: String,
    password: String,
    password_confirm: String,
    response: Option<ResponseResult<Route>>,
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
            response: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(Ok(redirect_to)) = &self.response {
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

                {
                    if let Some(Err(err)) = &self.response {
                        html! {
                            <p>{ err }</p>
                        }
                    } else {
                        html! {}
                    }
                }

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

                let scope = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let request = Client::new().post(path).json(&user_form);
                    let on_ok = ResponseAction::from(|_| Ok(Route::Home));
                    let requester = Requester::default();
                    let response = requester.make(request, on_ok).await;

                    scope.send_message(RegisterMsg::ResponseReceived(response));
                });
            }
            RegisterMsg::AppContextUpdated(context) => self.app_context = context,
            RegisterMsg::ResponseReceived(response) => {
                if response.is_ok() {
                    self.app_context
                        .borrow_mut()
                        .login(self.username.clone())
                        .expect("could not log in");
                }
                self.response = Some(response)
            }
        }
        true
    }
}
