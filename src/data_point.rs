use charts::{
    Chart, LineSeriesView, MarkerType, PointDatum, PointLabelPosition, ScaleBand, ScaleLinear,
};
use chrono::{Duration, NaiveDate};
use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub date: NaiveDate,
    pub acre_feet: f32,
}

impl std::cmp::PartialEq for DataPoint {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }

    fn ne(&self, other: &Self) -> bool {
        !DataPoint::eq(self, other)
    }
}
impl Eq for DataPoint {}

impl Ord for DataPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.date < other.date {
            other
        } else {
            self
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.date < other.date {
            self
        } else {
            other
        }
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        assert!(min <= max);
        if self.date < min.date {
            min
        } else if self.date > max.date {
            max
        } else {
            self
        }
    }
}

impl std::cmp::PartialOrd for DataPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        self.date < other.date
    }
    fn le(&self, other: &Self) -> bool {
        let eq = self.date == other.date;
        let lt = self.date < other.date;
        eq || lt
    }
    fn gt(&self, other: &Self) -> bool {
        !DataPoint::le(self, other)
    }
    fn ge(&self, other: &Self) -> bool {
        let eq = self.date == other.date;
        let gt = self.date > other.date;
        eq || gt
    }
}

impl PointDatum<String, f32> for DataPoint {
    fn get_x(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }
    fn get_y(&self) -> f32 {
        self.acre_feet
    }
    fn get_key(&self) -> String {
        String::new()
    }
}
