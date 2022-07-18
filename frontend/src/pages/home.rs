use std::sync::Arc;

use reqwest::Client;
use shared::models::predictions::PredictionWithDeltas;
use shared::{path_patterns, routes};
use yew::context::ContextHandle;
use yew::prelude::*;

use crate::components::{AppContext, Header, Loading, NewPrediction, PredictionPanel};
use crate::requests::{fully_qualified_path, replace_pattern, Requester, ResponseAction};
use crate::{InternalResponseError, ResponseResult};

pub enum HomeMsg {
    AppContextUpdated(AppContext),
    ReceivedResponse(ResponseResult<Vec<PredictionWithDeltas>>),
    DataUpdateRequired,
}

pub struct Home {
    app_context: AppContext,
    _context_listener: ContextHandle<AppContext>,
    prediction_response: Option<ResponseResult<Vec<PredictionWithDeltas>>>,
    client: Client,
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (app_context, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(HomeMsg::AppContextUpdated))
            .expect("no AppContext provided");

        Home {
            app_context,
            _context_listener,
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
                log::trace!("received response: {:?} {:?}", response, self.app_context);
                self.prediction_response = Some(response)
            }
            HomeMsg::AppContextUpdated(_context) => {}
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
        if let Some(username) = self.app_context.borrow_mut().username() {
            self.get_predictions(ctx, &username)
        } else {
            ctx.link().send_message(HomeMsg::ReceivedResponse(Err(
                InternalResponseError::Unauthorized,
            )))
        }
    }

    fn get_predictions(&self, ctx: &Context<Self>, username: &str) {
        let path = fully_qualified_path(
            &replace_pattern(routes::PREDICTIONS, path_patterns::PREDICTIONS, username)
                .expect("could not replace pattern in route"),
        )
        .expect("could not create path");

        let client = Arc::new(self.client.clone());
        let scope = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let request = client.get(path);
            let on_ok = ResponseAction::new(Box::new(|response| {
                Box::pin(async {
                    response
                        .json::<Vec<PredictionWithDeltas>>()
                        .await
                        .map_err(|err| {
                            InternalResponseError::ResponseAwaitError(
                                "predictions",
                                err.to_string(),
                            )
                        })
                })
            }));

            let requester = Requester::default();
            let response = requester.make(request, on_ok).await;

            scope.send_message(HomeMsg::ReceivedResponse(response));
        });
    }
}
