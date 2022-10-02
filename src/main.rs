mod data_point;
mod chart_model;
use charts::{
    Chart, LineSeriesView, MarkerType, PointDatum, PointLabelPosition, ScaleBand, ScaleLinear,
};
use chrono::{Duration, NaiveDate};
use data_point::DataPoint;
use chart_model::ChartModel;
use svg::node::element::Group;
use yew::prelude::*;

pub enum Events {
    StartDateUpdated(NaiveDate),
    EndDateUpdated(NaiveDate)
}

trait ToHtml {
    fn to_html(&self) -> Html;
}

impl ToHtml for Group {
    fn to_html(&self) -> Html {
        let svg_graph = String::from_utf8(self.to_string().as_bytes().to_vec()).unwrap();
        let xml_stuff =
            r#"xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" "#;
        web_sys::window()
            .and_then(|window| window.document())
            .map_or_else(
                || {
                    html! { <p>{ "Failed to resolve `document`." }</p> }
                },
                |document| match document.create_element("div") {
                    Ok(div) => {
                        let svg_obj = format!(
                            "<svg {} width=\"800\" height=\"600\">{}</svg>",
                            xml_stuff,
                            svg_graph
                        );
                        div.set_inner_html(&svg_obj);
                        yew::virtual_dom::VNode::VRef(div.into())
                    }
                    Err(e) => html! { <p>{ format!("{:?}", &e) }</p> },
                },
            )
    }
}


impl Component for ChartModel {
    type Message = Events;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ChartModel::generate_random_data(10)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.to_chart_svg().to_html()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Events::EndDateUpdated(new_end_date) => {
                self.update_end_date(new_end_date);
                true
            },
            Events::StartDateUpdated(new_start_date) => {
                self.update_start_date(new_start_date)
                true
            }
        }
        todo!()
    }
}


fn main() {}