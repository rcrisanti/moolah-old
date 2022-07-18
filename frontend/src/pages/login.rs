use reqwest::{Client, StatusCode};
use shared::models::{User, UserLoginRequestForm};
use shared::routes;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::{AppContext, Header};
use crate::errors::InternalResponseError;
use crate::requests::{fully_qualified_path, Requester, ResponseAction};
use crate::ResponseResult;

pub enum LoginMsg {
    AppContextUpdated(AppContext),
    UsernameChanged(String),
    PasswordChanged(String),
    Submitted,
    ResponseReceived(ResponseResult<Route>),
}

pub struct Login {
    app_context: AppContext,
    username: String,
    password: String,
    client: Client,
    response: Option<ResponseResult<Route>>,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_context, _) = ctx
            .link()
            .context(ctx.link().callback(LoginMsg::AppContextUpdated))
            .expect("no AppContext provided");

        Login {
            app_context,
            username: String::new(),
            password: String::new(),
            client: Client::new(),
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
                        if let Some(Err(error_msg)) = &self.response {
                            match error_msg {
                                InternalResponseError::Unauthorized => html! {
                                    <p>{ "incorrect credentials" }</p>
                                },
                                _ => html! {
                                    <p>{ error_msg }</p>
                                },
                            }

                        } else {
                            html! { <></> }
                        }
                    }
                    <div>
                        <label for="username">{ "username:" }</label>
                        <input id="username" type="text" placeholder="username" onchange={onchange_username} required=true/>
                    </div>
                    <div>
                        <label for="password">{ "password:" }</label>
                        <input id="password" type="password" placeholder="password" onchange={onchange_password} required=true/>
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

                let scope = ctx.link().clone();
                let client = Arc::new(self.client.clone());
                let password = self.password.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = client.put(path).json(&user_form).send().await.ok();

                    let response = if let Some(response) = response {
                        match response.status() {
                            StatusCode::OK => {
                                let user: User = response
                                    .json()
                                    .await
                                    .expect("could not retrieve user from response");

                                if user
                                    .verify_user(&user_form.username, &password)
                                    .expect("could not verify user")
                                {
                                    login_user(client, user).await
                                } else {
                                    Err(InternalResponseError::Unauthorized)
                                }
                            }
                            _ => response.text().await.map_or(
                                Err(InternalResponseError::Other(
                                    "could not get body text".into(),
                                )),
                                |err_text| {
                                    Err(InternalResponseError::ResponseAwaitError(
                                        "error text body",
                                        err_text,
                                    ))
                                },
                            ),
                        }
                    } else {
                        Err(InternalResponseError::Other(
                            "could not post login credentials:".into(),
                        ))
                    };

                    scope.send_message(LoginMsg::ResponseReceived(response));
                })
            }
            LoginMsg::AppContextUpdated(_) => todo!(),
            LoginMsg::ResponseReceived(response) => {
                if response.is_ok() {
                    self.app_context
                        .borrow_mut()
                        .login(self.username.clone())
                        .expect("could not log in");
                }

                self.response = Some(response);
            }
        }
        true
    }
}

async fn login_user(client: Arc<Client>, user: User) -> ResponseResult<Route> {
    let path = fully_qualified_path(routes::LOGIN.into())
        .expect("could not build fully qualified path for second request");
    let request = client.patch(path).json(&user);
    let on_ok = ResponseAction::new(Box::new(|_| Box::pin(async move { Ok(Route::Home) })));

    let requester = Requester::default();
    requester.make(request, on_ok).await
}
