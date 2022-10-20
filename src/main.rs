mod chart_model;
mod data_point;
use chart_model::ChartModel;
use chrono::{NaiveDate};
use js_sys::JsString;
use yew::prelude::*;
use gloo_console::log as gloo_log;
use wasm_bindgen::JsCast;

const DATE_FORMAT: &str = "%Y-%m-%d";
const END_DATE_NAME: &str = "end-date";
const START_DATE_NAME: &str = "start-date";
const DIV_END_DATE_NAME: &str = "div-end-date";
const DIV_START_DATE_NAME: &str = "div-start-date";

fn string_log(log_string: String) {
    let log_js_string: JsString = log_string.into();
    gloo_log!(log_js_string);
}

fn generic_callback(_event: Event, event_is_end: bool, dom_id_str: &str) -> Events {
        let updated_date =    
        web_sys::window()
            .and_then(|window| window.document())
            .map_or_else(
                || {
                    let log_string = "window document object not found.".to_string();
                    string_log(log_string);
                    NaiveDate::from_ymd(1992,3,26)
                },
                |document| match document.get_element_by_id(dom_id_str) {
                    Some(input) => {
                        let input_element = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        let date_value: String = input_element.value();
                        let result = NaiveDate::parse_from_str(&date_value, DATE_FORMAT).unwrap();
                        let log_string = format!("callback: {}", result.format(DATE_FORMAT));
                        string_log(log_string);
                        result
                    },
                    None => {
                        let log_string = format!("{} {}", dom_id_str, "dom object not found.");
                        string_log(log_string);
                        NaiveDate::from_ymd(1999,1,1)
                    }
                },
            );
            if event_is_end {
                Events::EndDateUpdated(updated_date)
            } else {
                Events::StartDateUpdated(updated_date)
            }
}

pub enum Events {
    StartDateUpdated(NaiveDate),
    EndDateUpdated(NaiveDate),
}

impl Component for ChartModel {
    type Message = Events;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ChartModel::generate_random_data(10)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // self.data.sort();
        let start_date = self.data.first().unwrap().date;
        let end_date = self.data.last().unwrap().date;
        let chart_str = self.to_chart_svg().to_string();
        let start_date_change_callback = ctx.link().callback(|event: Event| 
            generic_callback(event, false, START_DATE_NAME));
        let end_date_change_callback = ctx.link().callback(|event: Event| 
            generic_callback(event, true, END_DATE_NAME));
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
                        // https://www.brightec.co.uk/blog/svg-wouldnt-render
                        let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg").unwrap();
                        svg.set_attribute("id", "svg-chart").unwrap();
                        svg.set_attribute("width", "800").unwrap();
                        svg.set_attribute("height", "600").unwrap();
                        svg.set_inner_html(chart_str.as_str());
                        yew::virtual_dom::VNode::VRef(svg.into())
                    }
                },
            );
        html! {
            <div id="chart">
                {svg_vnode}
                <div id={DIV_START_DATE_NAME}>
                    <input onchange={start_date_change_callback} type="date" id={START_DATE_NAME} value={start_date.format(DATE_FORMAT).to_string()}/>
                </div>
                <div id={DIV_END_DATE_NAME}>
                    <input onchange={end_date_change_callback} type="date" id={END_DATE_NAME} value={end_date.format(DATE_FORMAT).to_string()}/>
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Events::EndDateUpdated(new_end_date) => {
                let end_date = self.data.last().unwrap().date;
                if end_date == new_end_date {
                    false
                } else {
                    let log_string = format!("update EndDateUpdated: {} from {}", new_end_date.format(DATE_FORMAT), end_date.format(DATE_FORMAT));
                    string_log(log_string);
                    self.update_end_date(new_end_date);
                    true
                }
            }
            Events::StartDateUpdated(new_start_date) => {
                let start_date = self.data.first().unwrap().date;
                if start_date == new_start_date {
                    false
                } else {
                    let log_string = format!("update StartDateUpdated: {} from {}", new_start_date.format(DATE_FORMAT), start_date.format(DATE_FORMAT));
                    string_log(log_string);
                    self.update_start_date(new_start_date);
                    true
                }
            }
        }
    }
}

fn main() {
    yew::start_app::<ChartModel>();
}
