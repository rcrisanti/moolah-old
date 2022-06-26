use gloo_dialogs::confirm;
use reqwest::Client;
use reqwest::StatusCode;
use shared::models::UserAccount;
use shared::routes;
use std::sync::Arc;
use thiserror::Error;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::Header;
use crate::services::requests::{fully_qualified_path, replace_pattern};
use crate::services::{identity_forget, identity_recall};

const PATH_PATTERN: &str = r"\{username\}";

#[derive(Error, Debug, Clone)]
pub enum AccountError {
    #[error("Not logged in")]
    Unauthorized,

    #[error("{0}")]
    Other(String),
}

pub enum AccountMsg {
    ReceivedResponse(Result<UserAccount, AccountError>),
    ResetPassword,
    DeleteAccountInitiated,
    DeleteAccountConfirmed,
    DeleteAccountSuccessful,
    DeleteAccountError(String),
}

pub struct Account {
    account: Option<Result<UserAccount, AccountError>>,
    client: Client,
    delete_account_err: Option<String>,
}

impl Component for Account {
    type Message = AccountMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let username = identity_recall();

        if let Some(username) = username {
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

            Account {
                account: None,
                client,
                delete_account_err: None,
            }
        } else {
            Account {
                account: Some(Err(AccountError::Unauthorized)),
                client: Client::new(),
                delete_account_err: None,
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
                        self.account
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
                identity_forget();
                self.account = Some(Err(AccountError::Unauthorized));
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

                <div>{ "not logged in" }</div>

                <div>
                    <Link<Route> to={Route::Login}>{ "login" }</Link<Route>>
                    <p>{ "or" }</p>
                    <Link<Route> to={Route::Register}>{ "register" }</Link<Route>>
                </div>
            </>
        }
    }

    fn view_loading(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header title="account" heading="account"/>

                <div>{ "loading..." }</div>
            </>
        }
    }
}
