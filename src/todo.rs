use chrono::prelude::*;
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
    pub fn set_date(mut self, date: NaiveDate) -> Self {
        self.date = date;
        self
    }
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
        format!(
            "{:2}d | {:15} | {:20} | {:5} ",
            self.get_days(),
            self.title,
            self.desc,
            &self.date.to_string()[5..].replace("-", "/")
        )
    }
    fn get_days(&self) -> i64 {
        let today = Local::now().naive_local().date();
        (self.date - today).num_days()
    }
}
impl PartialOrd for TodoBuilder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date.partial_cmp(&other.date)
    }
}
impl Ord for TodoBuilder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}
