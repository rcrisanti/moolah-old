use std::str::FromStr;

use chrono::{Duration, Local, NaiveDate, Weekday};
use reqwest::Client;
use shared::{
    models::{
        self,
        deltas::{app::repetition::MonthDay, db::DbDateRepetition},
        NewDbDelta, Repetition,
    },
    path_patterns, routes,
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::{
    components::AppContext,
    requests::{fully_qualified_path, replace_pattern, Requester, ResponseAction},
    ResponseResult,
};

fn input_callback<T, C>(ctx: &Context<T>, msg: C) -> Callback<Event>
where
    T: Component,
    C: Fn(HtmlInputElement) -> T::Message + Copy + 'static,
{
    ctx.link().batch_callback(move |ev: Event| {
        let target = ev.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        input.map(msg)
    })
}

fn submit_callback<T, C>(ctx: &Context<T>, msg: C) -> Callback<FocusEvent>
where
    T: Component,
    C: Fn() -> T::Message + Copy + 'static,
{
    ctx.link().callback(move |ev: FocusEvent| {
        ev.prevent_default();
        msg()
    })
}

#[derive(Debug, PartialEq, Properties)]
pub struct NewDeltaProps {
    pub prediction_id: i32,
    pub oncreate: Callback<()>,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unable to parse value ({0}) to f32")]
    Value(String),

    #[error("unable to parse positive uncertainty ({0}) to f32")]
    PositiveUncertainty(String),

    #[error("unable to parse negative uncertainty ({0}) to f32")]
    NegativeUncertainty(String),

    #[error("unable to parse repetition ({0}) to a repetition frequency")]
    Repetition(String),

    #[error("unable to parse start date ({0}) to a date")]
    StartDate(String),

    #[error("unable to parse end date ({0}) to a date")]
    EndDate(String),

    #[error("unable to parse monthly repeat day ({0}) to an i16 1-31")]
    RepeatDay(String),

    #[error("unable to parse weekly repeat weekday ({0}) to a weekday")]
    RepeatWeekday(String),
}

#[derive(Debug, thiserror::Error)]
pub enum RepetitionError {
    #[error("{0} repetition requires a value for {1}")]
    MissingFields(&'static str, &'static str),
}

pub enum NewDeltaMsg {
    Open(bool),
    NameChanged(String),
    ValueChanged(String),
    PosUncertaintyChanged(String),
    NegUncertaintyChanged(String),
    RepetitionChanged(String),
    StartDateChanged(String),
    EndDateChanged(String),
    RepeatDayChanged(String),
    RepeatWeekdayChanged(String),
    ErrorBuildingRepetition(RepetitionError),
    Submitted,
    ReceivedResponse(ResponseResult<()>),
}

pub struct NewDelta {
    app_context: AppContext,
    open: bool,
    parse_error: Option<ParseError>,
    repetition_error: Option<RepetitionError>,
    response: Option<ResponseResult<()>>,
    name: String,
    value: f32,
    positive_uncertainty: f32,
    negative_uncertainty: f32,
    db_repetition: DbDateRepetition,
    start_on: NaiveDate,
    end_on: Option<NaiveDate>,
    repeat_day: Option<MonthDay>,
    repeat_weekday: Option<Weekday>,
}

impl Component for NewDelta {
    type Message = NewDeltaMsg;
    type Properties = NewDeltaProps;

    fn create(ctx: &Context<Self>) -> Self {
        let now = Local::now().naive_utc().date();

        let (app_context, _) = ctx
            .link()
            .context(Callback::noop())
            .expect("no AppContext provided");

        NewDelta {
            app_context,
            open: false,
            parse_error: None,
            name: String::new(),
            value: 0.,
            positive_uncertainty: 0.,
            negative_uncertainty: 0.,
            db_repetition: DbDateRepetition::Monthly,
            start_on: now,
            end_on: Some(now + Duration::days(31)),
            repeat_day: Some(MonthDay::new(1).unwrap()),
            repeat_weekday: None,
            repetition_error: None,
            response: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NewDeltaMsg::Open(open) => self.open = open,
            NewDeltaMsg::NameChanged(name) => self.name = name,
            NewDeltaMsg::ValueChanged(value) => {
                if let Ok(value) = value.parse::<f32>() {
                    self.value = value;
                } else {
                    self.parse_error = Some(ParseError::Value(value));
                }
            }
            NewDeltaMsg::PosUncertaintyChanged(pos_unc) => {
                if let Ok(value) = pos_unc.parse::<f32>() {
                    self.positive_uncertainty = value;
                } else {
                    self.parse_error = Some(ParseError::PositiveUncertainty(pos_unc));
                }
            }
            NewDeltaMsg::NegUncertaintyChanged(neg_unc) => {
                if let Ok(value) = neg_unc.parse::<f32>() {
                    self.negative_uncertainty = value;
                } else {
                    self.parse_error = Some(ParseError::NegativeUncertainty(neg_unc));
                }
            }
            NewDeltaMsg::RepetitionChanged(repetition) => {
                log::debug!("repetition changed");
                if let Ok(repetition) = repetition.clone().try_into() {
                    self.db_repetition = repetition;
                    log::debug!("repetition changed to {:?}", self.db_repetition);
                } else {
                    self.parse_error = Some(ParseError::Repetition(repetition))
                }
            }
            NewDeltaMsg::StartDateChanged(date) => {
                log::info!("start date {}", date);
                if let Ok(start) = NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                    self.start_on = start;
                } else {
                    self.parse_error = Some(ParseError::StartDate(date))
                }
            }
            NewDeltaMsg::EndDateChanged(date) => {
                if let Ok(end) = NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                    self.end_on = Some(end);
                } else {
                    self.parse_error = Some(ParseError::EndDate(date))
                }
            }
            NewDeltaMsg::RepeatDayChanged(day) => {
                if let Ok(day) = day.parse::<i16>() {
                    if let Ok(repeat_day) = MonthDay::new(day) {
                        self.repeat_day = Some(repeat_day);
                        return true;
                    }
                }

                self.parse_error = Some(ParseError::RepeatDay(day))
            }
            NewDeltaMsg::RepeatWeekdayChanged(weekday) => {
                if let Ok(weekday) = Weekday::from_str(&weekday) {
                    self.repeat_weekday = Some(weekday);
                } else {
                    self.parse_error = Some(ParseError::RepeatWeekday(weekday))
                }
            }
            NewDeltaMsg::Submitted => self.post_delta_if_logged_in(ctx),
            NewDeltaMsg::ErrorBuildingRepetition(error) => self.repetition_error = Some(error),
            NewDeltaMsg::ReceivedResponse(response) => {
                if response.is_ok() {
                    ctx.props().oncreate.emit(());
                    self.open = false;
                }
                self.response = Some(response);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.open {
            self.view_open(ctx)
        } else {
            self.view_closed(ctx)
        }
    }
}

// Sub-views
impl NewDelta {
    fn view_open(&self, ctx: &Context<Self>) -> Html {
        let onchange_name = input_callback(ctx, |input| NewDeltaMsg::NameChanged(input.value()));
        let onchange_value = input_callback(ctx, |input| NewDeltaMsg::ValueChanged(input.value()));
        let onchange_pos_unc = input_callback(ctx, |input| {
            NewDeltaMsg::PosUncertaintyChanged(input.value())
        });
        let onchange_neg_unc = input_callback(ctx, |input| {
            NewDeltaMsg::NegUncertaintyChanged(input.value())
        });
        let oninput_repetition = ctx.link().callback(|ev: InputEvent| {
            let event = ev.dyn_into::<Event>().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target = event_target.dyn_into::<HtmlSelectElement>().unwrap_throw();
            NewDeltaMsg::RepetitionChanged(target.value())
        });
        let onsubmit = submit_callback(ctx, || NewDeltaMsg::Submitted);
        let oncancel = ctx.link().callback(|_| NewDeltaMsg::Open(false));

        html! {
            <>
                { self.view_errors() }

                <form {onsubmit}>
                    <div>
                        <label for="name">{ "name:" }</label>
                        <input type="text" id="name" name="name" required=true placehold="name" onchange={onchange_name}/>
                    </div>
                    <div>
                        <label for="value">{ "value:" }</label>
                        <input type="number" id="value" name="value" required=true onchange={onchange_value}/>
                    </div>
                    <div>
                        <label for="pos-unc">{ "positive uncertainty:" }</label>
                        <input type="number" id="pos-unc" name="pos-unc" required=false onchange={onchange_pos_unc}/>
                    </div>
                    <div>
                        <label for="neg-unc">{ "negative uncertainty:" }</label>
                        <input type="number" id="neg-unc" name="neg-unc" required=false onchange={onchange_neg_unc}/>
                    </div>
                    <div>
                        <label for="repetition">{ "repetition:" }</label>
                        <select name="repetition" id="repetition" oninput={oninput_repetition}>
                            // <option value="none" selected=true disabled=true hidden=true required=true>{ "select an option" }</option>
                            <option value="monthly" selected=true>{ "monthly" }</option>
                            <option value="weekly">{ "weekly" }</option>
                            <option value="daily">{ "daily" }</option>
                            <option value="once">{ "once" }</option>
                        </select>
                    </div>
                    {
                        match self.db_repetition {
                            DbDateRepetition::Monthly => self.view_monthly(ctx),
                            DbDateRepetition::Weekly => self.view_weekly(ctx),
                            DbDateRepetition::Daily => self.view_daily(ctx),
                            DbDateRepetition::Once => self.view_once(ctx),
                        }
                    }
                    <div>
                        <input type="submit" value="create"/>
                        <input type="button" value="cancel" onclick={oncancel}/>
                    </div>
                </form>
            </>
        }
    }

    fn view_monthly(&self, ctx: &Context<Self>) -> Html {
        let onchange_start =
            input_callback(ctx, |input| NewDeltaMsg::StartDateChanged(input.value()));
        let onchange_end = input_callback(ctx, |input| NewDeltaMsg::EndDateChanged(input.value()));
        let onchange_monthday =
            input_callback(ctx, |input| NewDeltaMsg::RepeatDayChanged(input.value()));

        html! {
            <>
                <div>
                    <label for="monthly-start">{ "starting on:" }</label>
                    <input type="date" id="monthly-start" name="monthly-start" onchange={onchange_start}/>
                </div>
                <div>
                    <label for="monthly-end">{ "ending on:" }</label>
                    <input type="date" id="monthly-end" name="monthly-end" onchange={onchange_end}/>
                </div>
                <div>
                    <label for="monthly-repeat">{ "repeating on:" }</label>
                    <input type="number" id="monthly-repeat" name="monthly-repeat" min=1 max=31 step=1 onchange={onchange_monthday}/>
                </div>
            </>
        }
    }

    fn view_weekly(&self, ctx: &Context<Self>) -> Html {
        let onchange_start =
            input_callback(ctx, |input| NewDeltaMsg::StartDateChanged(input.value()));
        let onchange_end = input_callback(ctx, |input| NewDeltaMsg::EndDateChanged(input.value()));
        let oninput_weekday = ctx.link().callback(|ev: InputEvent| {
            let event = ev.dyn_into::<Event>().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target = event_target.dyn_into::<HtmlSelectElement>().unwrap_throw();
            NewDeltaMsg::RepeatWeekdayChanged(target.value())
        });

        html! {
            <>
                <div>
                    <label for="weekly-start">{ "starting on:" }</label>
                    <input type="date" id="weekly-start" name="weekly-start" onchange={onchange_start}/>
                </div>
                <div>
                    <label for="weekly-end">{ "ending on:" }</label>
                    <input type="date" id="weekly-end" name="weekly-end" onchange={onchange_end}/>
                </div>
                <div>
                    <label for="weekly-weekday">{ "repeating on:" }</label>
                    <select name="weekly-weekday" id="weekly-weekday" oninput={oninput_weekday}>
                        <option value="Mon" selected=true>{ "monday" }</option>
                        <option value="Tue">{ "tuesday" }</option>
                        <option value="Wed">{ "wednesday" }</option>
                        <option value="Thu">{ "thursday" }</option>
                        <option value="Fri">{ "friday" }</option>
                        <option value="Sat">{ "saturday" }</option>
                        <option value="Sun">{ "sunday" }</option>
                    </select>
                </div>
            </>
        }
    }

    fn view_daily(&self, ctx: &Context<Self>) -> Html {
        let onchange_start =
            input_callback(ctx, |input| NewDeltaMsg::StartDateChanged(input.value()));
        let onchange_end = input_callback(ctx, |input| NewDeltaMsg::EndDateChanged(input.value()));

        html! {
            <>
                <div>
                    <label for="daily-start">{ "starting on:" }</label>
                    <input type="date" id="daily-start" name="daily-start" onchange={onchange_start}/>
                </div>
                <div>
                    <label for="daily-end">{ "ending on:" }</label>
                    <input type="date" id="daily-end" name="daily-end" onchange={onchange_end}/>
                </div>
            </>
        }
    }

    fn view_once(&self, ctx: &Context<Self>) -> Html {
        let onchange = input_callback(ctx, |input| NewDeltaMsg::StartDateChanged(input.value()));

        html! {
            <div>
                <label for="once-on">{ "occurs on:" }</label>
                <input type="date" id="once-on" name="once-on" {onchange}/>
            </div>
        }
    }

    fn view_errors(&self) -> Html {
        if let Some(error) = &self.parse_error {
            html! {
                <p>{ error.to_string() }</p>
            }
        } else if let Some(error) = &self.repetition_error {
            html! {
                <p>{ error.to_string() }</p>
            }
        } else {
            html! {}
        }
    }

    fn view_closed(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| NewDeltaMsg::Open(true));

        html! {
            <i class="fa fa-plus" aria-hidden="true" {onclick}></i>
        }
    }
}

// Request functions
impl NewDelta {
    fn post_delta_if_logged_in(&self, ctx: &Context<Self>) {
        if let Some(username) = self.app_context.borrow_mut().username() {
            self.post_delta(ctx, username)
        }
    }

    fn post_delta(&self, ctx: &Context<Self>, username: &str) {
        let repetition = match self.db_repetition {
            DbDateRepetition::Monthly => {
                if self.end_on.is_none() {
                    Err(RepetitionError::MissingFields("monthly", "end date"))
                } else if self.repeat_day.is_none() {
                    Err(RepetitionError::MissingFields("monthly", "repeat day"))
                } else {
                    Ok(Repetition::Monthly {
                        from: self.start_on,
                        to: self.end_on.unwrap(),
                        repeat_on_day: self.repeat_day.unwrap(),
                    })
                }
            }
            DbDateRepetition::Weekly => {
                if self.end_on.is_none() {
                    Err(RepetitionError::MissingFields("weekly", "end date"))
                } else if self.repeat_weekday.is_none() {
                    Err(RepetitionError::MissingFields("weekly", "repeat weekday"))
                } else {
                    Ok(Repetition::Weekly {
                        from: self.start_on,
                        to: self.end_on.unwrap(),
                        repeat_on_weekday: self.repeat_weekday.unwrap(),
                    })
                }
            }
            DbDateRepetition::Daily => {
                if self.end_on.is_none() {
                    Err(RepetitionError::MissingFields("daily", "end date"))
                } else {
                    Ok(Repetition::Daily {
                        from: self.start_on,
                        to: self.end_on.unwrap(),
                    })
                }
            }
            DbDateRepetition::Once => Ok(Repetition::Once { on: self.start_on }),
        };

        if let Err(repetition_err) = repetition {
            ctx.link()
                .send_message(NewDeltaMsg::ErrorBuildingRepetition(repetition_err));
            return;
        }

        let new_db_delta: NewDbDelta = models::NewDelta::new(
            ctx.props().prediction_id,
            self.name.clone(),
            self.value,
            self.positive_uncertainty,
            self.negative_uncertainty,
            repetition.unwrap(),
        )
        .into();

        let path = fully_qualified_path(
            &replace_pattern(routes::DELTAS, path_patterns::DELTAS, username)
                .expect("could not replace pattern"),
        )
        .expect("could not create fully qualified path");

        let scope = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let request = Client::new().post(path).json(&new_db_delta);
            let on_ok = ResponseAction::from(|_| Ok(()));
            let requester = Requester::default();
            let response = requester.make(request, on_ok).await;

            scope.send_message(NewDeltaMsg::ReceivedResponse(response));
        })
    }
}
