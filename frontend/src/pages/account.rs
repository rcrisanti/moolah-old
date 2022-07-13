use chrono::{DateTime, Local, Utc};
use gloo_dialogs::confirm;
use reqwest::Client;
use reqwest::StatusCode;
use shared::models::UserAccount;
use shared::routes;
use std::sync::Arc;
use thiserror::Error;
use yew::prelude::*;

use crate::components::AppContext;
use crate::components::{Header, Loading, Unauthorized};
use crate::services::requests::{fully_qualified_path, replace_pattern};

const PATH_PATTERN: &str = r"\{username\}";
const DATETIME_FORMAT: &str = "%a %h %d %Y %r %Z";

#[derive(Error, Debug, Clone)]
pub enum AccountError {
    #[error("Not logged in")]
    Unauthorized,

    #[error("{0}")]
    Other(String),
}

pub enum AccountMsg {
    AppContextUpdated(AppContext),
    ReceivedResponse(Result<UserAccount, AccountError>),
    ResetPassword,
    DeleteAccountInitiated,
    DeleteAccountConfirmed,
    DeleteAccountSuccessful,
    DeleteAccountError(String),
}

pub struct Account {
    app_context: AppContext,
    account: Option<Result<UserAccount, AccountError>>,
    client: Client,
    delete_account_err: Option<String>,
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
            delete_account_err: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(username) = self.app_context.borrow().current_username() {
                self.get_account(ctx, &username)
            } else {
                ctx.link()
                    .callback(|_| AccountMsg::ReceivedResponse(Err(AccountError::Unauthorized)))
                    .emit(0)
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.account {
            Some(Ok(_account)) => self.view_logged_in(ctx),
            Some(Err(_err)) => self.view_not_logged_in(ctx),
            None => self.view_loading(ctx),
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
                    ctx.link()
                        .callback(|_| AccountMsg::DeleteAccountConfirmed)
                        .emit(0);
                }
            }
            AccountMsg::DeleteAccountConfirmed => {
                let path = fully_qualified_path(
                    replace_pattern(
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
                let scope = Arc::new(ctx.link().clone());
                wasm_bindgen_futures::spawn_local(async move {
                    let response = client
                        .delete(path)
                        .send()
                        .await
                        .expect("could not get account");

                    match response.status() {
                        StatusCode::OK => {
                            scope
                                .callback(move |_| AccountMsg::DeleteAccountSuccessful)
                                .emit(0);
                        }
                        _ => {
                            let err = response.text().await.expect("could not get body text");
                            scope
                                .callback(move |_| AccountMsg::DeleteAccountError(err.clone()))
                                .emit(0);
                        }
                    };
                });
            }
            AccountMsg::DeleteAccountError(err) => self.delete_account_err = Some(err),
            AccountMsg::DeleteAccountSuccessful => {
                // identity_forget();
                self.app_context.borrow_mut().logout();
                self.account = Some(Err(AccountError::Unauthorized));
            }
            AccountMsg::AppContextUpdated(context) => {
                if context.borrow().current_username()
                    == self.app_context.borrow().current_username()
                {
                    // self.app_context = context;
                    return false;
                }
            }
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
                            if let Some(delete_account_err) = &self.delete_account_err {
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
            replace_pattern(routes::ACCOUNT, PATH_PATTERN, username)
                .expect("could not replace pattern in route"),
        )
        .expect("could not create path");

        let client = Client::new();
        let scope = Arc::new(ctx.link().clone());
        let arc_client = Arc::new(client.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let response = arc_client
                .get(path)
                .send()
                .await
                .expect("could not get account");

            let response_account = match response.status() {
                StatusCode::OK => {
                    let account: UserAccount = response
                        .json()
                        .await
                        .expect("could not get account from response");
                    Ok(account)
                }
                StatusCode::UNAUTHORIZED => Err(AccountError::Unauthorized),
                _ => Err(AccountError::Other(
                    response.text().await.expect("could not get body text"),
                )),
            };

            scope
                .callback(move |_| AccountMsg::ReceivedResponse(response_account.clone()))
                .emit(0);
        });
    }
}
