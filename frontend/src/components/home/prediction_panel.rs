use std::sync::Arc;

use reqwest::{Client, StatusCode};
use shared::{
    models::{predictions::PredictionWithDeltas, Prediction},
    path_patterns, routes,
};
use stylist::{css, YieldStyle};
use yew::prelude::*;

use crate::{
    components::AppContext,
    services::{replace_pattern, requests::fully_qualified_path},
    InternalResponseError,
};

#[derive(Properties, PartialEq)]
pub struct PredictionPanelProps {
    pub prediction: PredictionWithDeltas,
    pub ondelete: Callback<()>,
}

pub enum PredictionPanelMsg {
    AppContextUpdated(AppContext),
    DeletePrediction,
    ReceivedResponse(Result<(), InternalResponseError>),
}

pub struct PredictionPanel {
    app_context: AppContext,
    client: Client,
    response: Option<Result<(), InternalResponseError>>,
}

impl Component for PredictionPanel {
    type Message = PredictionPanelMsg;
    type Properties = PredictionPanelProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_context, _) = ctx
            .link()
            .context(ctx.link().callback(PredictionPanelMsg::AppContextUpdated))
            .expect("no AppContext provided");

        PredictionPanel {
            app_context,
            client: Client::new(),
            response: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_trash = ctx
            .link()
            .callback(|_| PredictionPanelMsg::DeletePrediction);

        html! {
            <>
                <h2>{ ctx.props().prediction.name() } <i class="fa fa-trash" onclick={onclick_trash}></i></h2>

                <div class={ self.style() }>
                    <h3>{ "deltas" }</h3>
                    {
                        if let Some(Err(err)) = &self.response {
                            html! {
                                <p>{err}</p>
                            }
                        } else {
                            html! {}
                        }
                    }

                    { self.view_delta_table(ctx) }
                </div>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PredictionPanelMsg::DeletePrediction => {
                self.delete_prediction_if_logged_in(ctx);
                ctx.props().ondelete.emit(());
            }
            PredictionPanelMsg::AppContextUpdated(context) => todo!(),
            PredictionPanelMsg::ReceivedResponse(response) => self.response = Some(response),
        }
        true
    }
}

impl PredictionPanel {
    fn view_delta_table(&self, ctx: &Context<Self>) -> Html {
        html! {
            <table>
                <tr>
                    <th>{ "name" }</th>
                    <th>{ "value" }</th>
                    <th>{ "dates" }</th>
                    <th>{ "uncertainty" }</th>
                </tr>
                {
                    ctx.props().prediction.deltas().into_iter().map(|delta| {
                        let value = format!(
                            "{}${:.2}",
                            if delta.value().is_sign_negative() {
                                "-"
                            } else {
                                ""
                            },
                            delta.value().abs()
                        );

                        let mut dates = delta.dates().clone();
                        dates.sort();

                        let dates_fmt = {
                            if dates.len() == 0 {
                                 "N/A".to_string()
                            } else if dates.len() == 1 {
                                dates.first().expect("should have 1 date").format("%x").to_string()
                            } else if dates.len() <= 3 {
                                dates.into_iter().map(|date| date.format("%x").to_string()).collect::<Vec<_>>().join(", ")
                            } else {
                                format!("{}, ...", dates[..3].into_iter().map(|date| date.format("%x").to_string()).collect::<Vec<_>>().join(", "))
                            }
                        };

                        let unc_fmt = {
                            if delta.positive_uncertainty() == delta.negative_uncertainty() {
                                format!("+/- ${:.2}", delta.positive_uncertainty())
                            } else {
                                format!("+${:.2} / -${:.2}", delta.positive_uncertainty(), delta.negative_uncertainty())
                            }
                        };

                        html! {
                            <tr key={ delta.id() }>
                                <td>{ delta.name() }</td>
                                <td>{ value }</td>
                                <td>{ dates_fmt }</td>
                                <td>{ unc_fmt }</td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </table>
        }
    }
}

impl PredictionPanel {
    fn delete_prediction_if_logged_in(&self, ctx: &Context<Self>) {
        if let Some(username) = self.app_context.borrow().current_username() {
            self.delete_prediction(ctx, &username);
        } else {
        }
    }

    fn delete_prediction(&self, ctx: &Context<Self>, username: &str) {
        let path = fully_qualified_path(
            replace_pattern(
                routes::PREDICTIONS,
                path_patterns::PREDICTIONS,
                username.into(),
            )
            .expect("could not replace pattern in route"),
        )
        .expect("could not create path");

        let prediction: Prediction = ctx.props().prediction.clone().into();

        let client = Arc::new(self.client.clone());
        let scope = Arc::new(ctx.link().clone());
        wasm_bindgen_futures::spawn_local(async move {
            log::debug!("deleting prediction: {:?}", prediction);
            let response = client.delete(path).json(&prediction).send().await.ok();

            let response = if let Some(response) = response {
                match response.status() {
                    StatusCode::OK => Ok(()),
                    StatusCode::UNAUTHORIZED => Err(InternalResponseError::Unauthorized),
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
                    "could not post new prediction".into(),
                ))
            };

            scope
                .callback(move |_| PredictionPanelMsg::ReceivedResponse(response.clone()))
                .emit(0);
        });
    }
}

impl YieldStyle for PredictionPanel {
    fn style_from(&self) -> stylist::StyleSource<'static> {
        css!(
            "table, th, td {
                border: 1px solid black;
                border-collapse: collapse;
                padding: 15px;
            }"
        )
    }
}
