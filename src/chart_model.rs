mod data_point;
use charts::{ScaleBand, ScaleLinear, LineSeriesView, MarkerType, Chart};
use data_point::DataPoint;
use chrono::{Duration, NaiveDate};
use svg::node::element::Group;
use yew::Html;
#[derive(Debug, Clone)]
pub struct ChartModel {
    label: String,
    data: Vec<DataPoint>,
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
                let acre_feet = idx as u32;
                DataPoint { date, acre_feet }
            })
            .collect();
        Self { label, data }
    }

    pub fn update_start_date(&mut self, new_start_date: NaiveDate) {
        self.data.sort();
        let start_date = self.data.first().unwrap();
        let end_date = self.data.last().unwrap();
        let data: Vec<DataPoint> = (start_date..end_date)
        .into_iter()
        .enumerate()
        .map(|(idx, d)| {
            let date = start_date + Duration::days(idx as i64);
            let acre_feet = idx as f32;
            DataPoint{date,acre_feet}
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
        let y = ScaleLinear::new()
            .set_domain(y_f32)
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

    fn get_end_date_input(&self) -> Html {
        todo!()
    }

    fn get_start_date_input(&self) -> Html {
        todo!()
    }
}