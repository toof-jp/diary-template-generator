use std::path::PathBuf;
use std::{env, fs};

use anyhow::{anyhow, Context, Result};
use diary_template_generator::*;
use time::{format_description, Date, OffsetDateTime};

fn main() -> Result<()> {
    let mut date_input: Option<String> = None;
    let mut obsidian_dir: Option<PathBuf> = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--obsidian" {
            let dir = args
                .next()
                .context("--obsidian requires a directory path")?;
            obsidian_dir = Some(PathBuf::from(dir));
        } else if date_input.is_none() {
            date_input = Some(arg);
        } else {
            return Err(anyhow!("Unexpected argument: {}", arg));
        }
    }

    let format = format_description::parse("[year]-[month]-[day]")?;

    let target_date: Date = if let Some(date_str) = date_input {
        Date::parse(&date_str, &format).context("Date argument must be in YYYY-mm-dd format")?
    } else {
        OffsetDateTime::now_local()?.date()
    };

    let first_day_of_week = get_first_day_of_week(&target_date)?;
    let diary_template = generate_diary_template(&first_day_of_week)?;

    if let Some(dir) = obsidian_dir {
        fs::create_dir_all(&dir).context("Failed to create obsidian directory")?;
        let file_name = format!("{}.md", first_day_of_week.format(&format)?);
        let file_path = dir.join(file_name);
        if file_path.exists() {
            return Err(anyhow!("File already exists: {}", file_path.display()));
        }
        let content = generate_diary_template_without_h1(&first_day_of_week)?;
        fs::write(&file_path, content)
            .with_context(|| format!("Failed to write {}", file_path.display()))?;
    } else {
        print!("{}", diary_template);
    }

    Ok(())
}
