use crate::todo;
use anyhow;
use chrono::NaiveDate;
use console::Term;
use dialoguer::Input;

pub fn input_todo(term: &Term) -> anyhow::Result<todo::TodoBuilder> {
    term.write_line("Input todo information")?;
    term.show_cursor()?;
    let mut todo = todo::TodoBuilder::new();

    let title: String = Input::new().with_prompt("title").interact_text()?;
    todo = todo.title(title);

    let date: String = Input::new()
        .with_prompt("[]w[]d or %Y-%m-%d")
        .allow_empty(true)
        .interact_text()?;

    match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(d) => todo = todo.set_date(d),
        Err(_) => {
            let (w, d) = parse_wd(date)?;
            todo = todo.weeks(w).days(d);
        }
    }

    let desc: String = Input::new()
        .with_prompt("desc")
        .allow_empty(true)
        .interact_text()?;
    todo = todo.desc(desc);

    term.clear_last_lines(4)?;
    Ok(todo)
}

fn parse_wd(wd: String) -> anyhow::Result<(u32, u32)> {
    // [NUM]w [NUM]d
    // w, d is only once
    let (mut w_flag, mut d_flag) = (false, false);
    let (mut w, mut d) = (0, 0);
    let mut num_s = String::new();
    let parse_error = "Only accept [INT]w [INT]d or %Y-%m-%d";

    for c in wd.chars() {
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

    Ok((w, d))
}
