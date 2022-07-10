use shared::models::predictions::PredictionWithDeltas;
use stylist::{css, YieldStyle};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PredictionPanelProps {
    pub prediction: PredictionWithDeltas,
}

pub struct PredictionPanel {}

impl Component for PredictionPanel {
    type Message = ();
    type Properties = PredictionPanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        PredictionPanel {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{ ctx.props().prediction.name.clone() }</h2>

                <div class={ self.style() }>
                    <h3>{ "deltas" }</h3>
                    { self.view_delta_table(ctx) }
                </div>
            </>
        }
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
                // {
                //     ctx.props().prediction.deltas.clone().into_iter().map(|delta| {
                //         let value = format!(
                //             "{}${:.2}",
                //             if delta.value.is_sign_negative() {
                //                 "-"
                //             } else {
                //                 ""
                //             },
                //             delta.value.abs()
                //         );

                //         let mut dates = delta.dates.clone();
                //         dates.sort();

                //         let dates_fmt = {
                //             if dates.len() == 0 {
                //                  "N/A".to_string()
                //             } else if dates.len() == 1 {
                //                 dates.first().expect("should have 1 date").format("%x").to_string()
                //             } else if dates.len() <= 3 {
                //                 dates.into_iter().map(|date| date.format("%x").to_string()).collect::<Vec<_>>().join(", ")
                //             } else {
                //                 format!("{}, ...", dates[..3].into_iter().map(|date| date.format("%x").to_string()).collect::<Vec<_>>().join(", "))
                //             }
                //         };

                //         let unc_fmt = {
                //             if delta.positive_uncertainty == delta.negative_uncertainty {
                //                 format!("+/- ${:.2}", delta.positive_uncertainty)
                //             } else {
                //                 format!("+${:.2} / -${:.2}", delta.positive_uncertainty, delta.negative_uncertainty)
                //             }
                //         };

                //         html! {
                //             <tr key={ delta.id }>
                //                 <td>{ delta.name }</td>
                //                 <td>{ value }</td>
                //                 <td>{ dates_fmt }</td>
                //                 <td>{ unc_fmt }</td>
                //             </tr>
                //         }
                //     }).collect::<Html>()
                // }
            </table>
        }
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
