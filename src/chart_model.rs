use crate::{data_point::DataPoint, string_log};
use charts::{Chart, LineSeriesView, MarkerType, ScaleBand, ScaleLinear};
use chrono::{Duration, NaiveDate};
use svg::node::element::Group;


#[derive(Debug, Clone)]
pub struct ChartModel {
    pub label: String,
    pub data: Vec<DataPoint>,
}

impl ChartModel {
    pub fn new(label: String, data: Vec<DataPoint>) -> Self {
        Self { label, data }
    }

    pub fn generate_random_data(sample_size: usize) -> Self {
        let start_date = NaiveDate::from_ymd(1989, 6, 22);
        let label = String::from("test");
        let data: Vec<DataPoint> = (0..sample_size)
            .into_iter()
            .map(|idx| {
                let date = start_date + Duration::days(idx as i64);
                let acre_feet = idx as f32;
                DataPoint { date, acre_feet }
            })
            .collect();
        Self { label, data }
    }

    pub fn update_start_date(&mut self, new_date: NaiveDate) {
        self.data.sort();
        let (start_date, end_date) = {
            let start = self.data.first().unwrap().date;
            let end = self.data.last().unwrap().date;
            if new_date < start {
                (new_date, start)
            } else if start <= new_date && new_date <= end {
                (start, new_date)
            } else {
                (start, new_date)
            }
        };
        let duration = ((end_date - start_date).num_days() + 1) as usize;
        let data: Vec<DataPoint> = start_date
            .iter_days()
            .take(duration)
            .enumerate()
            .map(|(idx, _d)| {
                let date = start_date + Duration::days(idx as i64);
                let acre_feet = idx as f32;
                DataPoint { date, acre_feet }
            })
            .collect();
        self.data = data;
    }

    pub fn update_end_date(&mut self, new_end_date: NaiveDate) {
        self.update_start_date(new_end_date);
    }

    pub fn to_chart_svg(&self) -> Group {
        let self_clone = self.clone();
        // Define chart related sizes.
        let width = 800;
        let height = 600;
        let (top, right, bottom, left) = (90, 40, 50, 60);

        // Create a band scale that will interpolate values in [0, 200] to values in the
        // [0, availableWidth] range (the width of the chart without the margins).
        let x = ScaleBand::new()
            .set_domain(
                self_clone
                    .data
                    .clone()
                    .into_iter()
                    .map(|x| x.date.format("%Y-%m-%d").to_string())
                    .collect(),
            )
            .set_range(vec![0, width - left - right]);

        // Create a linear scale that will interpolate values in [0, 100] range to corresponding
        // values in [availableHeight, 0] range (the height of the chart without the margins).
        // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
        // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
        // the range on Y axis for the chart to display as though its origin is at bottom left.
        let y_f32: Vec<f32> = self_clone
            .data
            .clone()
            .into_iter()
            .map(|x| x.acre_feet as f32)
            .collect();
        let y_max = y_f32.iter().clone().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let y_min = y_f32.iter().clone().fold(f32::INFINITY, |a, &b| a.min(b));
        let y_domain_log_string = format!("y_domain[min, max]: [{}, {}]", y_min, y_max);
        string_log(y_domain_log_string);
        let y_domain = vec![y_min, y_max];
        let y = ScaleLinear::new()
            .set_domain(y_domain)
            .set_range(vec![height - top - bottom, 0]);

        // You can use your own iterable as data as long as its items implement the `PointDatum` trait.

        let line_data = self_clone.data;
        let line_label = self_clone.label;
        // Create Line series view that is going to represent the data.
        let line_view = LineSeriesView::new()
            .set_x_scale(&x)
            .set_y_scale(&y)
            .set_marker_type(MarkerType::Circle)
            // .set_label_position(PointLabelPosition::N)
            .load_data(&line_data)
            .unwrap();

        // Generate and save the chart.
        let chart_svg = Chart::new()
            .set_width(width)
            .set_height(height)
            .set_margins(top, right, bottom, left)
            .add_title(line_label)
            .add_view(&line_view)
            .add_axis_bottom(&x)
            .add_axis_left(&y)
            .add_left_axis_label("Acrefeet")
            .add_bottom_axis_label("Date")
            .to_svg()
            .unwrap();
        chart_svg
    }
}
