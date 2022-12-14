use crate::todo;
use chrono::prelude::*;
use chrono::{Duration, NaiveDate};
use console::Term;
use dialoguer::Input;

pub fn input_todo(
    term: &Term,
    icontent: Option<String>,
    idate: Option<String>,
) -> anyhow::Result<todo::Todo> {
    let mut line_count = 0;
    term.write_line("TODO:")?;
    line_count += 1;
    term.show_cursor()?;

    let content = match icontent {
        Some(c) => c,
        None => {
            line_count += 1;
            let content: String = Input::new().with_prompt("content").interact_text()?;
            content
        }
    };

    let date = match idate {
        Some(d) => d,
        None => {
            line_count += 1;
            Input::new()
                .with_prompt("[INT]w[INT]d or MM/DD")
                .allow_empty(true)
                .interact_text()?
        }
    };

    term.clear_last_lines(line_count)?;
    Ok(todo::Todo::new(content, parse_date_input(date)?))
}

fn parse_date_input(date: String) -> anyhow::Result<NaiveDate> {
    let parse_error = "Only accept [INT]w[INT]d or MM/DD";
    let today = Local::now().naive_local().date();
    let year = today.year();

    // Parse MM/DD
    if date.contains('/') {
        let ary: Vec<&str> = date.split('/').collect();
        if ary.len() != 2 {
            return Err(anyhow::anyhow!(parse_error));
        }
        let month: u32 = ary[0].parse()?;
        let day: u32 = ary[1].parse()?;

        let date_str = if is_next_year_date(&today, month, day) {
            format!("{}-{}-{}", year + 1, month, day)
        } else {
            format!("{}-{}-{}", year, month, day)
        };

        Ok(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")?)
    }
    // Parse <INT>w<INT>d
    else {
        let (mut w_flag, mut d_flag) = (false, false);
        let (mut w, mut d) = (0, 0);
        let mut num_s = String::new();

        for c in date.chars() {
            match c {
                c @ '0'..='9' => num_s.push(c),
                'w' => {
                    if w_flag {
                        return Err(anyhow::anyhow!(parse_error));
                    } else {
                        w_flag = true;
                        w = num_s.parse::<u32>()?;
                        num_s = String::new();
                    }
                }
                'd' => {
                    if d_flag {
                        return Err(anyhow::anyhow!(parse_error));
                    } else {
                        d_flag = true;
                        d = num_s.parse::<u32>()?;
                        num_s = String::new();
                    }
                }
                ' ' => continue,
                _ => return Err(anyhow::anyhow!(parse_error)),
            }
        }
        Ok(today + Duration::weeks(w.into()) + Duration::days(d.into()))
    }
}

fn is_next_year_date(today: &NaiveDate, month: u32, day: u32) -> bool {
    (month < today.month()) || (month == today.month() && day < today.day())
}
