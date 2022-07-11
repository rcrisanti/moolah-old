use std::sync::Arc;

use chrono::{NaiveDate, Weekday};
use reqwest::{Client, StatusCode};
use shared::models::deltas::app::repetition::MonthDay;
use shared::models::predictions::PredictionWithDeltas;
use shared::models::{Delta, Repetition};
use shared::{path_patterns, routes};
use yew::prelude::*;

use crate::components::{Header, Loading, NewPrediction, PredictionPanel};
use crate::services::requests::fully_qualified_path;
use crate::services::{identity_recall, replace_pattern};

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
                        Some(Err(_err)) => self.view_not_logged_in(ctx),
                        None => self.view_loading(ctx),
                    }
                }
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HomeMsg::ReceivedResponse(response) => self.prediction_response = Some(response),
        }
        true
    }
}

impl Home {
    fn view_logged_in(&self, _ctx: &Context<Self>, predictions: Vec<PredictionWithDeltas>) -> Html {
        html! {
            <div>
                { format!("you have {} predictions created", predictions.len()) }

                <NewPrediction username={identity_recall().unwrap()} />

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

impl Home {
    fn get_predictions(&self, ctx: &Context<Self>, username: String) {
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
                    let preds: Vec<PredictionWithDeltas> = response
                        .json()
                        .await
                        .expect("could not get predictions from response");
                    Ok(preds)
                }
                StatusCode::UNAUTHORIZED => Err(HomeError::Unauthorized),
                _ => Err(HomeError::Other(
                    response.text().await.expect("could not get body text"),
                )),
            };

            scope
                .callback(move |_| HomeMsg::ReceivedResponse(response_preds.clone()))
                .emit(0);
        });

        // For testing purposes
        // let response_preds = vec![
        //     PredictionWithDeltas {
        //         id: 1,
        //         username: identity_recall().unwrap(),
        //         name: String::from("prediction 1"),
        //         deltas: vec![
        //             Delta::new(
        //                 1,
        //                 1,
        //                 String::from("delta 1"),
        //                 32.4,
        //                 0.0,
        //                 0.0,
        //                 Repetition::Monthly {
        //                     from: NaiveDate::from_ymd(2022, 3, 21),
        //                     to: NaiveDate::from_ymd(2022, 5, 21),
        //                     repeat_on_day: MonthDay::new(21).unwrap(),
        //                 },
        //             ),
        //             Delta::new(
        //                 2,
        //                 1,
        //                 String::from("delta 2"),
        //                 -125.,
        //                 5.0,
        //                 5.0,
        //                 Repetition::Weekly {
        //                     from: NaiveDate::from_ymd(2022, 1, 1),
        //                     to: NaiveDate::from_ymd(2022, 1, 12),
        //                     repeat_on_weekday: Weekday::Mon,
        //                 },
        //             ),
        //         ],
        //     },
        //     PredictionWithDeltas {
        //         id: 2,
        //         username: identity_recall().unwrap(),
        //         name: String::from("pred2"),
        //         deltas: vec![Delta::new(
        //             3,
        //             2,
        //             String::from("delta 1"),
        //             32.4,
        //             0.0,
        //             0.0,
        //             Repetition::Daily {
        //                 from: NaiveDate::from_ymd(2022, 5, 21),
        //                 to: NaiveDate::from_ymd(2022, 5, 25),
        //             },
        //         )],
        //     },
        // ];
        // ctx.link()
        //     .callback(move |_| HomeMsg::ReceivedResponse(Ok(response_preds.clone())))
        //     .emit(0);
    }
}
