use std::sync::Arc;

use chrono::NaiveDate;
use reqwest::{Client, StatusCode};
use shared::models::predictions::PredictionWithDeltas;
use shared::models::{deltas::DateRepetition, Delta};
use shared::routes;
use yew::prelude::*;

use crate::components::{Header, Loading, PredictionPanel};
use crate::services::requests::fully_qualified_path;
use crate::services::{identity_recall, replace_pattern};

const PATH_PATTERN: &str = r"\{username\}";

#[derive(thiserror::Error, Debug, Clone)]
pub enum HomeError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("{0}")]
    Other(String),
}

pub enum HomeMsg {
    ReceivedResponse(Result<Vec<PredictionWithDeltas>, HomeError>),
}

pub struct Home {
    prediction_response: Option<Result<Vec<PredictionWithDeltas>, HomeError>>,
    client: Client,
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Home {
            prediction_response: None,
            client: Client::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(username) = identity_recall() {
                self.get_predictions(ctx, username)
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header heading="moolah" />

                {
                    match &self.prediction_response {
                        Some(Ok(predictions)) => self.view_logged_in(ctx, predictions.to_vec()),
                        Some(Err(err)) => self.view_not_logged_in(ctx),
                        None => self.view_loading(ctx),
                    }
                }
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::ReceivedResponse(response) => self.prediction_response = Some(response),
        }
        true
    }
}

impl Home {
    fn get_predictions(&self, ctx: &Context<Self>, username: String) {
        // let path = fully_qualified_path(
        //     replace_pattern(routes::PREDICTIONS, PATH_PATTERN, username)
        //         .expect("could not replace pattern in route"),
        // )
        // .expect("could not create path");

        // let client = Arc::new(self.client.clone());
        // let scope = Arc::new(ctx.link().clone());
        // wasm_bindgen_futures::spawn_local(async move {
        //     let response = client
        //         .get(path)
        //         .send()
        //         .await
        //         .expect("could not get predictions");

        //     let response_preds = match response.status() {
        //         StatusCode::OK => {
        //             let preds: Vec<PredictionWithDeltas> = response
        //                 .json()
        //                 .await
        //                 .expect("could not get predictions from response");
        //             Ok(preds)
        //         }
        //         StatusCode::UNAUTHORIZED => Err(HomeError::Unauthorized),
        //         _ => Err(HomeError::Other(
        //             response.text().await.expect("could not get body text"),
        //         )),
        //     };

        //     scope
        //         .callback(move |_| HomeMsg::ReceivedResponse(response_preds.clone()))
        //         .emit(0);
        // });

        // For testing purposes
        let response_preds = vec![
            PredictionWithDeltas {
                id: 1,
                username: identity_recall().unwrap(),
                name: String::from("prediction 1"),
                deltas: vec![
                    Delta {
                        id: 1,
                        prediction_id: 1,
                        name: String::from("delta 1"),
                        value: 32.4,
                        positive_uncertainty: 0.0,
                        negative_uncertainty: 0.0,
                        dates: vec![
                            NaiveDate::from_ymd(2022, 3, 21),
                            NaiveDate::from_ymd(2022, 4, 21),
                            NaiveDate::from_ymd(2022, 5, 21),
                        ],
                        repetition: DateRepetition::Monthly(21),
                    },
                    Delta {
                        id: 2,
                        prediction_id: 1,
                        name: String::from("delta 2"),
                        value: -125.,

                        positive_uncertainty: 5.0,
                        negative_uncertainty: 5.0,
                        dates: vec![
                            NaiveDate::from_ymd(2022, 1, 1),
                            NaiveDate::from_ymd(2022, 1, 8),
                        ],
                        repetition: DateRepetition::Weekly(7),
                    },
                ],
            },
            PredictionWithDeltas {
                id: 2,
                username: identity_recall().unwrap(),
                name: String::from("pred2"),
                deltas: vec![Delta {
                    id: 3,
                    prediction_id: 2,
                    name: String::from("delta 1"),
                    value: 32.4,
                    positive_uncertainty: 0.0,
                    negative_uncertainty: 0.0,
                    dates: vec![
                        NaiveDate::from_ymd(2022, 5, 21),
                        NaiveDate::from_ymd(2022, 5, 22),
                        NaiveDate::from_ymd(2022, 5, 23),
                        NaiveDate::from_ymd(2022, 5, 24),
                        NaiveDate::from_ymd(2022, 5, 25),
                    ],
                    repetition: DateRepetition::Daily,
                }],
            },
        ];
        ctx.link()
            .callback(move |_| HomeMsg::ReceivedResponse(Ok(response_preds.clone())))
            .emit(0);
    }

    fn view_logged_in(&self, ctx: &Context<Self>, predictions: Vec<PredictionWithDeltas>) -> Html {
        html! {
            <div>
                { format!("you have {} predictions created", predictions.len()) }

                {
                    predictions.into_iter().map(|pred| html!{
                        <PredictionPanel prediction={pred} />
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
