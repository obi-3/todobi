use crate::todo;
use anyhow;
use chrono::NaiveDate;
use console::Term;
use dialoguer::Input;

pub fn input_todo(term: &Term, icontent: Option<String>, idate: Option<String>) -> anyhow::Result<todo::TodoBuilder> {
    term.write_line("Input todo information")?;
    term.show_cursor()?;
    let mut todo = todo::TodoBuilder::new();

    let content = match icontent {
        Some(c) => c,
        None => {
            let content: String = Input::new().with_prompt("title").interact_text()?;
            content
        }
    };
    todo = todo.content(content);

    let date = match idate {
        Some(d) => d,
        None => {
            Input::new()
                .with_prompt("[]w[]d or %Y-%m-%d")
                .allow_empty(true)
                .interact_text()?
        }
    };
    todo = todo.set_date(parse_date_input(date)?);

    // term.clear_last_lines(4)?;
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

fn parse_date_input(data: String) -> anyhow::Result<NaiveDate> {
    todo!();
}
