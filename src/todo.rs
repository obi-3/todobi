use chrono::prelude::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    content: String,
    pub date: NaiveDate,
    done: bool,
}

impl Todo {
    pub fn new(content: String, date: NaiveDate) -> Self {
        Self {
            content,
            // date: Local::now().naive_local().date(),
            date,
            done: false,
        }
    }
    pub fn toggle(mut self) -> Self {
        self.done = !self.done;
        self
    }
    pub fn is_done(&self) -> bool {
        self.done
    }
    pub fn format(&self, len: Option<usize>) -> String {
        match len {
            None => {
                format!(
                    "{:2}d | {} | {:5}",
                    self.get_days(),
                    self.content,
                    &self.date.to_string()[5..].replace('-', "/")
                )
            }
            Some(len) => {
                let self_len = self.get_content_len();
                let sub = len - self_len;
                let mut s = String::new();
                s += &format!("{:2}d | ", self.get_days());
                for _ in 0..sub / 2 {
                    s.push(' ');
                }
                s += &self.content;
                for _ in 0..sub / 2 {
                    s.push(' ');
                }
                if sub % 2 == 1 {
                    s.push(' ');
                }
                s += &format!(" | {:5}", &self.date.to_string()[5..].replace('-', "/"));

                s
            }
        }
    }
    fn get_days(&self) -> i64 {
        let today = Local::now().naive_local().date();
        (self.date - today).num_days()
    }
    pub fn get_content_len(&self) -> usize {
        self.content.len()
    }
}

impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date.partial_cmp(&other.date)
    }
}
impl Ord for Todo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}
