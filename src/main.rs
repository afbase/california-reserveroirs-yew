mod chart_model;
mod data_point;
use chart_model::ChartModel;
use charts::{
    Chart, LineSeriesView, MarkerType, PointDatum, PointLabelPosition, ScaleBand, ScaleLinear,
};
use chrono::{Duration, NaiveDate, Local};
use data_point::DataPoint;
use js_sys::JsString;
use svg::node::element::Group;
use yew::{events, prelude::*, utils::print_node, virtual_dom::VNode};
use gloo_console::log as gloo_log;
use wasm_bindgen::{prelude::*, JsCast};

pub enum Events {
    StartDateUpdated(NaiveDate),
    EndDateUpdated(NaiveDate),
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
                            xml_stuff, svg_graph
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
        // self.data.sort();
        let start_date = self.data.first().unwrap().date;
        let end_date = self.data.last().unwrap().date;
        // let xml_stuff =
        //     r#"xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" "#;
        // let start_date_input = format!(r#"<div name="div-startdate"> <input type="date" name="startdate" value="{}"/> </div>"#, start_date.format("%Y-%m-%d").to_string());
        // let end_date_input = format!(r#"<div name="div-enddate"> <input type="date" name="enddate" value="{}"/> </div>"#, end_date.format("%Y-%m-%d").to_string());
        let chart_str = self.to_chart_svg().to_string();
        let start_date_change_callback = ctx.link().callback(|event: Event| {
            let event_string = event.value_of().to_string().as_string().unwrap();
            let new_event_date = NaiveDate::parse_from_str(&event_string, "%Y-%m-%d").unwrap();
            Events::StartDateUpdated(new_event_date)
        });
        let end_date_change_callback = ctx.link().callback(|event: Event| {
        let updated_end_date =    
        web_sys::window()
            .and_then(|window| window.document())
            .map_or_else(
                || {
                    NaiveDate::from_ymd(1992,3,26)
                },
                |document| match document.get_element_by_id("enddate") {
                    Some(input) => {
                        let input_element = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        let date_value: String = input_element.value();
                        let result = NaiveDate::parse_from_str(&date_value, "%Y-%m-%d").unwrap();
                        let log_string = format!("callback: {}", result.format("%Y-%m-%d"));
                        let log_js_string: JsString = log_string.into();
                        gloo_log!(log_js_string);
                        result
                    },
                    None => {
                        NaiveDate::from_ymd(1999,1,1)
                    }
                },
            );
            // gloo_log!(event);
            // web_sys::console::log(event.value_of());
            // web_sys::console::log(event.value_of().to_string());
            // web_sys::console::log(event.value_of().to_string().as_string().unwrap());

            // old code
            // let event_string = event.value_of().to_string();
            // gloo_log!(event_string);
            // let date_event_option = event.value_of().to_string().as_string();
            // let date_event = date_event_option.unwrap();
            // let new_event_date = NaiveDate::parse_from_str(&date_event, "%Y-%m-%d").unwrap();
            let log_string = format!("{}", updated_end_date.format("%Y-%m-%d"));
            let log_js_string: JsString = log_string.into();
            gloo_log!(log_js_string);
            Events::EndDateUpdated(updated_end_date)
        });
        let svg_vnode = web_sys::window()
            .and_then(|window| window.document())
            .map_or_else(
                || {
                    html! { <p>{ "Failed to resolve `document`." }</p> }
                },
                |document| match document.get_element_by_id("svg-chart") {
                    Some(svg) => {
                        svg.set_inner_html(chart_str.as_str());
                        yew::virtual_dom::VNode::VRef(svg.into())
                    }
                    None => {
                        let svg = document.create_element("svg").unwrap();
                        svg.set_attribute("id", "svg-chart").unwrap();
                        svg.set_inner_html(chart_str.as_str());
                        yew::virtual_dom::VNode::VRef(svg.into())
                    }
                },
            );
        html! {
            <div name="chart">
                {svg_vnode}
                <div name="div-startdate">
                    <input onchange={start_date_change_callback} type="date" name="startdate" value={start_date.format("%Y-%m-%d").to_string()}/>
                </div>
                <div name="div-enddate">
                    <input onchange={end_date_change_callback} type="date" name="enddate" value={end_date.format("%Y-%m-%d").to_string()}/>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Events::EndDateUpdated(new_end_date) => {
                self.update_end_date(new_end_date);
                true
            }
            Events::StartDateUpdated(new_start_date) => {
                self.update_start_date(new_start_date);
                true
            }
        }
    }
}

fn main() {
    yew::start_app::<ChartModel>();
}
