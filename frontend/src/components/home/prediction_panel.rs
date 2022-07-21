use std::sync::Arc;

use reqwest::Client;
use shared::{
    models::{
        deltas::app::repetition::MonthDay, predictions::PredictionWithDeltas, Prediction,
        Repetition,
    },
    path_patterns, routes,
};
use stylist::{css, YieldStyle};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::requests::{fully_qualified_path, replace_pattern, Requester, ResponseAction};
use crate::{
    components::{AppContext, NewDelta},
    ResponseResult,
};

const DATE_FMT: &str = "%x";

#[derive(Properties, PartialEq)]
pub struct PredictionPanelProps {
    pub prediction: PredictionWithDeltas,
    pub ondelete: Callback<()>,
    pub onupdate: Callback<()>,
}

pub enum PredictionPanelMsg {
    AppContextUpdated(AppContext),
    DeletePrediction,
    UpdatePredictionNameRequested,
    PredictionNameChanged(String),
    PredictionNameChangeSubmitted,
    PredictionNameChangeCanceled,
    ReceivedDeleteResponse(ResponseResult<()>),
    ReceivedUpdateResponse(ResponseResult<()>),
}

pub struct PredictionPanel {
    app_context: AppContext,
    client: Client,
    delete_response: Option<ResponseResult<()>>,
    update_response: Option<ResponseResult<()>>,
    open: bool,
    updated_prediction_name: String,
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
            delete_response: None,
            update_response: None,
            open: false,
            updated_prediction_name: String::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oncreate_delta = ctx
            .link()
            .callback(|_| PredictionPanelMsg::ReceivedUpdateResponse(Ok(())));

        html! {
            <>
                {
                    if self.open {
                        self.view_open_prediction_name(ctx)
                    } else {
                        self.view_closed_prediction_name(ctx)
                    }
                }


                <div class={ self.style() }>
                    <h3>{ "deltas" }</h3>
                    {
                        if let Some(Err(err)) = &self.delete_response {
                            html! {
                                <p>{err}</p>
                            }
                        } else {
                            html! {}
                        }
                    }

                    { self.view_delta_table(ctx) }

                    <NewDelta prediction_id={ctx.props().prediction.id()} oncreate={oncreate_delta}/>
                </div>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PredictionPanelMsg::DeletePrediction => {
                self.delete_prediction_if_logged_in(ctx);
                log::trace!("delete prediction requested");
            }
            PredictionPanelMsg::AppContextUpdated(_) => todo!(),
            PredictionPanelMsg::ReceivedDeleteResponse(response) => {
                if response.is_ok() {
                    self.delete_response = Some(response);
                    ctx.props().ondelete.emit(());
                    log::trace!("delete prediction completed");
                } else {
                    self.delete_response = Some(response);
                    log::error!("error deleting prediction");
                }
            }
            PredictionPanelMsg::UpdatePredictionNameRequested => self.open = true,
            PredictionPanelMsg::PredictionNameChanged(name) => self.updated_prediction_name = name,
            PredictionPanelMsg::PredictionNameChangeSubmitted => {
                self.update_prediction_if_logged_in(ctx);
                log::trace!("update prediction requested");
            }
            PredictionPanelMsg::PredictionNameChangeCanceled => self.open = false,
            PredictionPanelMsg::ReceivedUpdateResponse(response) => {
                self.open = false;

                if response.is_ok() {
                    self.update_response = Some(response);
                    ctx.props().onupdate.emit(());
                    log::trace!("update prediction completed");
                } else {
                    self.update_response = Some(response);
                    log::error!("error updating prediction");
                    return false;
                }
            }
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
                    <th>{ "uncertainty" }</th>
                    <th>{ "occurs" }</th>
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

                        // let mut dates = delta.dates().clone();
                        // dates.sort();

                        // let dates_fmt = {
                        //     if dates.len() == 0 {
                        //          "N/A".to_string()
                        //     } else if dates.len() == 1 {
                        //         dates.first().expect("should have 1 date").format("%x").to_string()
                        //     } else if dates.len() <= 3 {
                        //         dates.into_iter().map(|date| date.format("%x").to_string()).collect::<Vec<_>>().join(", ")
                        //     } else {
                        //         format!("{}, ...", dates[..3].into_iter().map(|date| date.format("%x").to_string()).collect::<Vec<_>>().join(", "))
                        //     }
                        // };

                        let unc_fmt = {
                            if delta.positive_uncertainty() == delta.negative_uncertainty() {
                                format!("+/- ${:.2}", delta.positive_uncertainty())
                            } else {
                                format!("+${:.2} / -${:.2}", delta.positive_uncertainty(), delta.negative_uncertainty())
                            }
                        };

                        let dates_fmt = match delta.repetition() {
                            Repetition::Monthly { from, to, repeat_on_day } => {
                                let day = format!(
                                    "{}{}",
                                    repeat_on_day,
                                    if repeat_on_day > MonthDay::new(28).unwrap() {
                                        " (or final)"
                                    } else {
                                        ""
                                    }
                                );
                                format!(
                                    "the {} day of each month from {} to {}",
                                    day,
                                    from.format(DATE_FMT),
                                    to.format(DATE_FMT)
                                )
                            },
                            Repetition::Weekly { from, to, repeat_on_weekday } => {
                                format!(
                                    "every {} from {} to {}",
                                    repeat_on_weekday.to_string(),
                                    from.format(DATE_FMT),
                                    to.format(DATE_FMT)
                                )
                            }
                            Repetition::Daily { from, to } => {
                                format!("every day from {} to {}", from.format(DATE_FMT), to.format(DATE_FMT))
                            }
                            Repetition::Once { on } => format!("one time on {}", on.format(DATE_FMT)),
                        };

                        html! {
                            <tr key={ delta.id() }>
                                <td>{ delta.name() }</td>
                                <td>{ value }</td>
                                <td>{ unc_fmt }</td>
                                <td>{ dates_fmt }</td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </table>
        }
    }

    fn view_open_prediction_name(&self, ctx: &Context<Self>) -> Html {
        let onchange_predname = ctx.link().batch_callback(|ev: Event| {
            let target = ev.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| PredictionPanelMsg::PredictionNameChanged(input.value()))
        });

        let onsubmit = ctx.link().callback(|ev: FocusEvent| {
            ev.prevent_default();
            PredictionPanelMsg::PredictionNameChangeSubmitted
        });

        let onclick_cancel = ctx
            .link()
            .callback(|_| PredictionPanelMsg::PredictionNameChangeCanceled);

        html! {
            <div>
                {
                    if let Some(Err(err)) = &self.delete_response {
                        html!{
                            <div>
                                {format!("error updating prediction name: {}", err)}
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <form {onsubmit}>
                    <input type="text" placeholder={ctx.props().prediction.name().to_owned()} onchange={onchange_predname}/>
                    <input type="submit" value="update"/>
                    <input type="button" value="cancel" onclick={onclick_cancel}/>
                </form>
            </div>
        }
    }

    fn view_closed_prediction_name(&self, ctx: &Context<Self>) -> Html {
        let onclick_delete = ctx
            .link()
            .callback(|_| PredictionPanelMsg::DeletePrediction);

        let onclick_edit = ctx
            .link()
            .callback(|_| PredictionPanelMsg::UpdatePredictionNameRequested);

        html! {
            <h2>
                { ctx.props().prediction.name() }
                <i class="fa fa-pencil" aria-hidden="true" onclick={onclick_edit}></i>
                <i class="fa fa-trash" aria-hidden="true" onclick={onclick_delete}></i>
            </h2>
        }
    }
}

impl PredictionPanel {
    fn delete_prediction_if_logged_in(&self, ctx: &Context<Self>) {
        if let Some(username) = self.app_context.borrow_mut().username() {
            self.delete_prediction(ctx, &username);
        }
    }

    fn path(&self, username: &str) -> String {
        fully_qualified_path(
            &replace_pattern(routes::PREDICTIONS, path_patterns::PREDICTIONS, username)
                .expect("could not replace pattern in route"),
        )
        .expect("could not create path")
    }

    fn delete_prediction(&self, ctx: &Context<Self>, username: &str) {
        let path = self.path(username);

        let prediction: Prediction = ctx.props().prediction.clone().into();

        let client = Arc::new(self.client.clone());
        let scope = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            log::debug!("deleting prediction: {:?}", prediction);
            let request = client.delete(path).json(&prediction);
            let on_ok = ResponseAction::from(|_| Ok(()));
            let requester = Requester::default();
            let response = requester.make(request, on_ok).await;

            scope.send_message(PredictionPanelMsg::ReceivedDeleteResponse(response.clone()));
        });
    }

    fn update_prediction_if_logged_in(&self, ctx: &Context<Self>) {
        if let Some(username) = self.app_context.borrow_mut().username() {
            self.update_prediction(ctx, &username);
        }
    }

    fn update_prediction(&self, ctx: &Context<Self>, username: &str) {
        let path = self.path(username);

        let mut prediction: Prediction = ctx.props().prediction.clone().into();
        prediction.update_name(self.updated_prediction_name.clone());

        let client = Arc::new(self.client.clone());
        let scope = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            log::debug!("updating prediction: {:?}", prediction);
            let request = client.patch(path).json(&prediction);
            let on_ok = ResponseAction::from(|_| Ok(()));
            let requester = Requester::default();
            let response = requester.make(request, on_ok).await;

            scope.send_message(PredictionPanelMsg::ReceivedUpdateResponse(response.clone()));
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
