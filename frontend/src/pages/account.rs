use chrono::{DateTime, Local, Utc};
use gloo_dialogs::confirm;
use reqwest::Client;
use shared::models::UserAccount;
use shared::routes;
use std::sync::Arc;
use yew::prelude::*;
use yew_router::components::Redirect;

use crate::app::Route;
use crate::components::AppContext;
use crate::components::{Header, Loading, Unauthorized};
use crate::requests::{fully_qualified_path, replace_pattern, Requester, ResponseAction};
use crate::InternalResponseError;
use crate::ResponseResult;

const PATH_PATTERN: &str = r"\{username\}";
const DATETIME_FORMAT: &str = "%a %h %d %Y %r %Z";

pub enum AccountMsg {
    AppContextUpdated(AppContext),
    ReceivedResponse(ResponseResult<UserAccount>),
    ResetPassword,
    DeleteAccountInitiated,
    DeleteAccountConfirmed,
    ReceivedDeleteAccountResponse(ResponseResult<Route>),
}

pub struct Account {
    app_context: AppContext,
    account: Option<ResponseResult<UserAccount>>,
    client: Client,
    delete_response: Option<ResponseResult<Route>>,
}

impl Component for Account {
    type Message = AccountMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_context, _) = ctx
            .link()
            .context(ctx.link().callback(AccountMsg::AppContextUpdated))
            .expect("no AppContext provided");

        Account {
            app_context,
            account: None,
            client: Client::new(),
            delete_response: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(username) = self.app_context.borrow_mut().username() {
                self.get_account(ctx, &username)
            } else {
                ctx.link()
                    .callback(|_| {
                        AccountMsg::ReceivedResponse(Err(InternalResponseError::Unauthorized))
                    })
                    .emit(0)
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(Ok(redirect_route)) = self.delete_response {
            html! {
                <Redirect<Route> to={redirect_route.clone()} />
            }
        } else {
            match &self.account {
                Some(Ok(_account)) => self.view_logged_in(ctx),
                Some(Err(_err)) => self.view_not_logged_in(ctx),
                None => self.view_loading(ctx),
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AccountMsg::ReceivedResponse(resp) => self.account = Some(resp),
            AccountMsg::ResetPassword => todo!(),
            AccountMsg::DeleteAccountInitiated => {
                let delete_confirmed = confirm(
                    "Are you sure you want to delete your moolah account? This action cannot be undone.",
                );

                if delete_confirmed {
                    ctx.link().send_message(AccountMsg::DeleteAccountConfirmed);
                }
            }
            AccountMsg::DeleteAccountConfirmed => {
                let path = fully_qualified_path(
                    &replace_pattern(
                        routes::ACCOUNT,
                        PATH_PATTERN,
                        &self
                            .account
                            .as_ref()
                            .expect("should have account")
                            .as_ref()
                            .expect("should have account")
                            .username
                            .clone(),
                    )
                    .expect("could not replace pattern in route"),
                )
                .expect("could not create path");

                let client = Arc::new(self.client.clone());
                let scope = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let request = client.delete(path);
                    let on_ok = ResponseAction::from(|_| Ok(Route::Home));
                    let requester = Requester::default();
                    let response = requester.make(request, on_ok).await;

                    scope.send_message(AccountMsg::ReceivedDeleteAccountResponse(response));
                });
            }

            AccountMsg::ReceivedDeleteAccountResponse(response) => {
                if response.is_ok() {
                    self.app_context.borrow_mut().logout();
                }
                self.delete_response = Some(response);
            }
            AccountMsg::AppContextUpdated(_context) => {}
        }
        true
    }
}

impl Account {
    fn view_logged_in(&self, ctx: &Context<Self>) -> Html {
        let account = self
            .account
            .as_ref()
            .expect("this should never panic - should have response")
            .as_ref()
            .expect("this should never panic - should have account");

        let onclick_reset_password = ctx.link().callback(|_| AccountMsg::ResetPassword);
        let onclick_delete_account = ctx.link().callback(|_| AccountMsg::DeleteAccountInitiated);

        let created = DateTime::<Utc>::from_utc(account.created, Utc)
            .with_timezone(&Local)
            .format(DATETIME_FORMAT)
            .to_string();
        let last_login = DateTime::<Utc>::from_utc(account.last_login, Utc)
            .with_timezone(&Local)
            .format(DATETIME_FORMAT)
            .to_string();

        html! {
            <>
                <Header title="account" heading={account.username.clone()}/>

                <div>
                    <h2>{ "email" }</h2>
                    <p>{ account.email.clone() }</p>
                </div>

                <div>
                    <h2>{ "password" }</h2>
                    <button onclick={onclick_reset_password}>{ "reset password" }</button>
                </div>

                <div>
                    <h2>{ "account" }</h2>
                    <div>{ format!("created: {}", created) }</div>
                    <div>{ format!("last_login: {}", last_login) }</div>
                    <div>
                        <button onclick={onclick_delete_account}>{ "delete account" }</button>
                        {
                            if let Some(Err(delete_account_err)) = &self.delete_response {
                                html!{<p>{ delete_account_err }</p>}
                            } else {
                                html!{}
                            }
                        }
                    </div>
                </div>
            </>
        }
    }

    fn view_not_logged_in(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header title="account" heading="account"/>

                <Unauthorized />
            </>
        }
    }

    fn view_loading(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header title="account" heading="account"/>

                <Loading />
            </>
        }
    }
}

impl Account {
    fn get_account(&self, ctx: &Context<Self>, username: &str) {
        let path = fully_qualified_path(
            &replace_pattern(routes::ACCOUNT, PATH_PATTERN, username)
                .expect("could not replace pattern in route"),
        )
        .expect("could not create path");

        let scope = ctx.link().clone();
        let client = Arc::new(Client::new());
        wasm_bindgen_futures::spawn_local(async move {
            let request = client.get(path);
            let on_ok = ResponseAction::new(Box::new(|response| {
                Box::pin(async {
                    let account: UserAccount = response
                        .json()
                        .await
                        .expect("could not get account from response");
                    Ok(account)
                })
            }));

            let requester = Requester::default();
            let response = requester.make(request, on_ok).await;

            scope.send_message(AccountMsg::ReceivedResponse(response));
        });
    }
}
