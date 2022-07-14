use std::sync::Arc;

use reqwest::{Client, StatusCode};
use shared::models::predictions::PredictionWithDeltas;
use shared::{path_patterns, routes};
use yew::prelude::*;

use crate::components::{AppContext, Header, Loading, NewPrediction, PredictionPanel};
use crate::services::replace_pattern;
use crate::services::requests::fully_qualified_path;
use crate::InternalResponseError;

pub enum HomeMsg {
    AppContextUpdated(AppContext),
    ReceivedResponse(Result<Vec<PredictionWithDeltas>, InternalResponseError>),
    DataUpdateRequired,
}

pub struct Home {
    app_context: AppContext,
    prediction_response: Option<Result<Vec<PredictionWithDeltas>, InternalResponseError>>,
    client: Client,
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_context, _) = ctx
            .link()
            .context(ctx.link().callback(HomeMsg::AppContextUpdated))
            .expect("no AppContext provided");

        Home {
            app_context,
            prediction_response: None,
            client: Client::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.get_predictions_if_logged_in(ctx)
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header heading="moolah" />

                {
                    match &self.prediction_response {
                        Some(Ok(predictions)) => {
                            log::trace!("viewing logged in");
                            self.view_logged_in(ctx, predictions.to_vec())
                        },
                        Some(Err(_err)) => {
                            log::trace!("viewing not logged in");
                            self.view_not_logged_in(ctx)
                        },
                        None => {
                            log::trace!("viewing loading");
                            self.view_loading(ctx)
                        },
                    }
                }
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::ReceivedResponse(response) => {
                log::trace!("received response");
                self.prediction_response = Some(response)
            }
            HomeMsg::AppContextUpdated(context) => {
                if context.borrow().current_username()
                    == self.app_context.borrow().current_username()
                {
                    // self.app_context = context;
                    return false;
                }
            }
            HomeMsg::DataUpdateRequired => self.get_predictions_if_logged_in(ctx),
        }
        true
    }
}

impl Home {
    fn view_logged_in(&self, ctx: &Context<Self>, predictions: Vec<PredictionWithDeltas>) -> Html {
        let on_data_update = ctx.link().callback(|_| HomeMsg::DataUpdateRequired);

        html! {
            <div>
                { format!("you have {} predictions created", predictions.len()) }

                <NewPrediction oncreate={on_data_update.clone()} />

                {
                    predictions.into_iter().map(|pred| html!{
                        <PredictionPanel prediction={pred.clone()} ondelete={on_data_update.clone()}/>
                    }).collect::<Html>()
                }
            </div>
        }
    }

    fn view_not_logged_in(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{ "here is the main page" }</div>
        }
    }

    fn view_loading(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Loading />
        }
    }
}

impl Home {
    fn get_predictions_if_logged_in(&self, ctx: &Context<Self>) {
        if let Some(username) = self.app_context.borrow().current_username() {
            self.get_predictions(ctx, &username)
        } else {
            ctx.link()
                .callback(|_| HomeMsg::ReceivedResponse(Err(InternalResponseError::Unauthorized)))
                .emit(0)
        }
    }

    fn get_predictions(&self, ctx: &Context<Self>, username: &str) {
        let path = fully_qualified_path(
            replace_pattern(routes::PREDICTIONS, path_patterns::PREDICTIONS, username)
                .expect("could not replace pattern in route"),
        )
        .expect("could not create path");

        let client = Arc::new(self.client.clone());
        let scope = Arc::new(ctx.link().clone());
        wasm_bindgen_futures::spawn_local(async move {
            let response = client
                .get(path)
                .send()
                .await
                .expect("could not get predictions");

            let response_preds = match response.status() {
                StatusCode::OK => {
                    let preds = response
                        .json::<Vec<PredictionWithDeltas>>()
                        .await
                        .expect("could not get predictions from response");
                    Ok(preds)
                }
                StatusCode::UNAUTHORIZED => Err(InternalResponseError::Unauthorized),
                _ => Err(InternalResponseError::Other(
                    response.text().await.expect("could not get body text"),
                )),
            };

            scope
                .callback(move |_| HomeMsg::ReceivedResponse(response_preds.clone()))
                .emit(0);
        });
    }
}
