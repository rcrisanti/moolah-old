use std::sync::Arc;

use reqwest::{Client, StatusCode};
use shared::{
    models::{self, PredictionWithDeltas},
    path_patterns, routes,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    components::AppContext,
    services::{replace_pattern, requests::fully_qualified_path},
};

#[derive(Properties, PartialEq)]
pub struct NewPredictionProps {
    pub oncreate: Callback<PredictionWithDeltas>,
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum NewPredictionError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("error receiving new prediction from response: {0}")]
    ResponsePredictionError(String),

    #[error("{0}")]
    Other(String),
}

pub enum NewPredictionMsg {
    AppContextUpdated(AppContext),
    Open(bool),
    PredictionNameChanged(String),
    Submitted,
    FailedToPost(NewPredictionError),
    ReceivedResponse(Result<PredictionWithDeltas, NewPredictionError>),
}

pub struct NewPrediction {
    app_context: AppContext,
    prediction_name: String,
    client: Client,
    response_error: Option<NewPredictionError>,
    open: bool,
}

impl Component for NewPrediction {
    type Message = NewPredictionMsg;
    type Properties = NewPredictionProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_context, _) = ctx
            .link()
            .context(ctx.link().callback(NewPredictionMsg::AppContextUpdated))
            .expect("no AppContext provided");

        NewPrediction {
            app_context,
            prediction_name: String::new(),
            client: Client::new(),
            response_error: None,
            open: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange_predname = ctx.link().batch_callback(|ev: Event| {
            let target = ev.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| NewPredictionMsg::PredictionNameChanged(input.value()))
        });

        let onsubmit = ctx.link().callback(|ev: FocusEvent| {
            ev.prevent_default();
            NewPredictionMsg::Submitted
        });

        let onclick_new_pred = ctx.link().callback(|_| NewPredictionMsg::Open(true));
        let onclick_cancel = ctx.link().callback(|_| NewPredictionMsg::Open(false));

        if self.open {
            html! {
                <div>
                    <h3>{ "new prediction" }</h3>
                    {
                        if let Some(err) = &self.response_error {
                            html!{
                                <div>
                                    {format!("error creating new prediction: {}", err)}
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <form {onsubmit}>
                        <input type="text" placeholder="prediction name" onchange={onchange_predname}/>
                        <input type="submit" value="create"/>
                        <input type="button" value="cancel" onclick={onclick_cancel}/>
                    </form>
                </div>
            }
        } else {
            html! {
                <div>
                    <button onclick={onclick_new_pred}>{ "new prediction" }</button>
                </div>
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NewPredictionMsg::PredictionNameChanged(name) => {
                log::trace!("prediction name updated");
                self.prediction_name = name
            }
            NewPredictionMsg::Submitted => {
                if let Some(username) = self.app_context.borrow().current_username() {
                    self.post_prediction(ctx, &username)
                } else {
                    ctx.link()
                        .callback(|_| {
                            NewPredictionMsg::FailedToPost(NewPredictionError::Unauthorized)
                        })
                        .emit(0)
                }
            }
            NewPredictionMsg::ReceivedResponse(response) => match response {
                Ok(new_pred) => {
                    log::info!("successfully received response after posting new prediction");
                    ctx.props().oncreate.emit(new_pred);
                    self.open = false;
                    // self.app_context.borrow_mut().force_reload_predictions();
                }
                Err(err) => self.response_error = Some(err),
            },
            NewPredictionMsg::Open(open) => self.open = open,
            NewPredictionMsg::AppContextUpdated(context) => {
                if context.borrow().current_username()
                    == self.app_context.borrow().current_username()
                {
                    // self.app_context = context;
                    return false;
                }
            }
            NewPredictionMsg::FailedToPost(reason) => self.response_error = Some(reason),
        }
        true
    }
}

impl NewPrediction {
    fn post_prediction(&self, ctx: &Context<Self>, username: &str) {
        let path = fully_qualified_path(
            replace_pattern(
                routes::PREDICTIONS,
                path_patterns::PREDICTIONS,
                username.into(),
            )
            .expect("could not replace pattern in route"),
        )
        .expect("could not create path");

        let new_prediction =
            models::NewPrediction::new(username.into(), self.prediction_name.clone());

        let client = Arc::new(self.client.clone());
        let scope = Arc::new(ctx.link().clone());
        wasm_bindgen_futures::spawn_local(async move {
            log::debug!("posting new prediction: {:?}", new_prediction);
            let response = client.post(path).json(&new_prediction).send().await.ok();

            let response = if let Some(response) = response {
                match response.status() {
                    StatusCode::OK => {
                        response
                            .json::<PredictionWithDeltas>()
                            .await
                            .map_err(|err| {
                                NewPredictionError::ResponsePredictionError(err.to_string())
                            })
                    }
                    StatusCode::UNAUTHORIZED => Err(NewPredictionError::Unauthorized),
                    _ => Err(NewPredictionError::Other(
                        response
                            .text()
                            .await
                            .unwrap_or("could not get body text".into()),
                    )),
                }
            } else {
                Err(NewPredictionError::Other(
                    "could not post new prediction".into(),
                ))
            };

            scope
                .callback(move |_| NewPredictionMsg::ReceivedResponse(response.clone()))
                .emit(0);
        });
    }
}
