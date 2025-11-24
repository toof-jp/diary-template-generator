use std::fmt::Write as _;

use anyhow::{Context, Result};
use time::Weekday::*;
use time::{format_description, Date};
use wasm_bindgen::prelude::*;

pub fn get_first_day_of_week(date: &Date) -> Result<Date> {
    let mut date = Date::clone(date);

    while date.weekday() != Monday {
        date = date.previous_day().context("")?;
    }

    Ok(date)
}

#[wasm_bindgen]
pub fn get_first_day_of_week_from_str(date_str: &str) -> Result<String, JsError> {
    let format = format_description::parse("[year]-[month]-[day]")
        .map_err(|e| JsError::new(&e.to_string()))?;

    let date = Date::parse(date_str, &format).map_err(|e| JsError::new(&e.to_string()))?;

    get_first_day_of_week(&date)
        .map_err(|e| JsError::new(&e.to_string()))
        .and_then(|d| d.format(&format).map_err(|e| JsError::new(&e.to_string())))
}

pub fn generate_diary_template(date: &Date) -> Result<String> {
    build_diary_template(date, true)
}

pub fn generate_diary_template_without_h1(date: &Date) -> Result<String> {
    build_diary_template(date, false)
}

fn build_diary_template(date: &Date, include_h1: bool) -> Result<String> {
    let mut diary_template = String::new();
    let mut date = Date::clone(date);
    let format = format_description::parse("[year]-[month]-[day]")?;

    if include_h1 {
        writeln!(&mut diary_template, "# {}", &date.format(&format)?)?;
    }

    for _ in 0..7 {
        write!(&mut diary_template, "## {}\n\n", &date.format(&format)?)?;
        date = date.next_day().context("")?;
    }

    Ok(diary_template)
}

#[wasm_bindgen]
pub fn generate_diary_template_from_str(date_str: &str) -> Result<String, JsError> {
    let format = format_description::parse("[year]-[month]-[day]")
        .map_err(|e| JsError::new(&e.to_string()))?;

    let date = Date::parse(date_str, &format).map_err(|e| JsError::new(&e.to_string()))?;

    generate_diary_template(&date).map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen]
pub fn generate_diary_template_without_h1_from_str(date_str: &str) -> Result<String, JsError> {
    let format = format_description::parse("[year]-[month]-[day]")
        .map_err(|e| JsError::new(&e.to_string()))?;

    let date = Date::parse(date_str, &format).map_err(|e| JsError::new(&e.to_string()))?;

    generate_diary_template_without_h1(&date).map_err(|e| JsError::new(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use time::macros::date;

    use super::*;

    #[test]
    fn test_get_first_day_of_week() -> Result<()> {
        let date = date!(2021 - 12 - 26);
        assert_eq!(date.weekday(), Sunday);
        assert_eq!(get_first_day_of_week(&date)?, date!(2021 - 12 - 20));

        let date = date!(2021 - 12 - 27);
        assert_eq!(date.weekday(), Monday);
        assert_eq!(get_first_day_of_week(&date)?, date!(2021 - 12 - 27));

        let date = date!(2021 - 12 - 28);
        assert_eq!(date.weekday(), Tuesday);
        assert_eq!(get_first_day_of_week(&date)?, date!(2021 - 12 - 27));

        let date = date!(2021 - 12 - 29);
        assert_eq!(date.weekday(), Wednesday);
        assert_eq!(get_first_day_of_week(&date)?, date!(2021 - 12 - 27));

        let date = date!(2021 - 12 - 30);
        assert_eq!(date.weekday(), Thursday);
        assert_eq!(get_first_day_of_week(&date)?, date!(2021 - 12 - 27));

        let date = date!(2021 - 12 - 31);
        assert_eq!(date.weekday(), Friday);
        assert_eq!(get_first_day_of_week(&date)?, date!(2021 - 12 - 27));

        let date = date!(2022 - 01 - 01);
        assert_eq!(date.weekday(), Saturday);
        assert_eq!(get_first_day_of_week(&date)?, date!(2021 - 12 - 27));

        let date = date!(2022 - 01 - 02);
        assert_eq!(date.weekday(), Sunday);
        assert_eq!(get_first_day_of_week(&date)?, date!(2021 - 12 - 27));

        let date = date!(2022 - 01 - 03);
        assert_eq!(date.weekday(), Monday);
        assert_eq!(get_first_day_of_week(&date)?, date!(2022 - 01 - 03));

        Ok(())
    }

    #[test]
    fn test_generate_diary_template() -> Result<()> {
        let date = date!(2021 - 12 - 27);

        assert_eq!(
            generate_diary_template(&date)?,
            "\
# 2021-12-27
## 2021-12-27

## 2021-12-28

## 2021-12-29

## 2021-12-30

## 2021-12-31

## 2022-01-01

## 2022-01-02

"
        );

        Ok(())
    }

    #[test]
    fn test_generate_diary_template_without_h1() -> Result<()> {
        let date = date!(2021 - 12 - 27);

        assert_eq!(
            generate_diary_template_without_h1(&date)?,
            "\
## 2021-12-27

## 2021-12-28

## 2021-12-29

## 2021-12-30

## 2021-12-31

## 2022-01-01

## 2022-01-02

"
        );

        Ok(())
    }
}
