use chrono::prelude::*;
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd)]
pub struct TodoBuilder {
    title: String,
    pub date: NaiveDate,
    desc: String,
    done: bool,
}

impl TodoBuilder {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            date: Local::now().naive_local().date(),
            desc: String::new(),
            done: false,
        }
    }
    pub fn title<I: Into<String>>(mut self, title: I) -> Self {
        self.title = title.into();
        self
    }
    pub fn weeks<I: Into<i64>>(mut self, weeks: I) -> Self {
        self.date += Duration::weeks(weeks.into());
        self
    }
    pub fn days<I: Into<i64>>(mut self, days: I) -> Self {
        self.date += Duration::days(days.into());
        self
    }
    // pub fn ymd<Y: Into<i32>, D: Into<u32>>(mut self, year: Y, month: D, day: D) -> Self {
    //     self.date = NaiveDate::from_ymd(year.into(), month.into(), day.into());
    //     self
    // }
    pub fn desc<I: Into<String>>(mut self, desc: I) -> Self {
        self.desc = desc.into();
        self
    }
    pub fn toggle(mut self) -> Self {
        self.done = !self.done;
        self
    }
    pub fn is_done(&self) -> bool {
        self.done
    }
    pub fn to_string(&self) -> String {
        format!("{}|{}|{}", self.title, self.desc, self.date)
    }
}

impl Ord for TodoBuilder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}
