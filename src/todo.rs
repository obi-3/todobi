use chrono::prelude::*;
use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TodoBuilder {
    content: String,
    pub date: NaiveDate,
    done: bool,
}

impl TodoBuilder {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            date: Local::now().naive_local().date(),
            done: false,
        }
    }
    pub fn content<I: Into<String>>(mut self, title: I) -> Self {
        self.content = title.into();
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
    pub fn toggle(mut self) -> Self {
        self.done = !self.done;
        self
    }
    pub fn is_done(&self) -> bool {
        self.done
    }
    pub fn to_string(&self) -> String {
        format!(
            "{:2}d | {:50} | {:5} ",
            self.get_days(),
            self.content,
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
