use crate::todo;
use anyhow;
use console::Term;
use dialoguer::Input;

pub fn input_todo(term: &Term) -> anyhow::Result<todo::TodoBuilder> {
    term.write_line("Input todo information")?;
    term.show_cursor()?;
    let title: String = Input::new().with_prompt("title").interact_text()?;

    let date: String = Input::new()
        .with_prompt("[]w[]d")
        .allow_empty(true)
        .interact_text()?;
    let (w, d) = parse_wd(date)?;

    let desc: String = Input::new()
        .with_prompt("desc")
        .allow_empty(true)
        .interact_text()?;

    term.clear_last_lines(4)?;
    let todo = todo::TodoBuilder::new()
        .title(title)
        .weeks(w)
        .days(d)
        .desc(desc);
    term.hide_cursor()?;
    Ok(todo)
}

fn parse_wd(wd: String) -> anyhow::Result<(u32, u32)> {
    // [NUM]w [NUM]d
    // w, d is only once
    let (mut w_flag, mut d_flag) = (false, false);
    let (mut w, mut d) = (0, 0);
    let mut num_s = String::new();

    for c in wd.chars() {
        match c {
            c @ '0'..='9' => num_s.push(c),
            'w' => {
                if w_flag {
                    return Err(anyhow::anyhow!("Only accept [INT]w [INT]d"));
                } else {
                    w_flag = true;
                    w = num_s.parse::<u32>()?;
                    num_s = String::new();
                }
            }
            'd' => {
                if d_flag {
                    return Err(anyhow::anyhow!("Only accept [INT]w [INT]d"));
                } else {
                    d_flag = true;
                    d = num_s.parse::<u32>()?;
                    num_s = String::new();
                }
            }
            ' ' => continue,
            _ => return Err(anyhow::anyhow!("Only accept [INT]w [INT]d")),
        }
    }

    Ok((w, d))
}
