use anyhow::{Context, Result};
use std::fmt::Write as _;
use time::format_description;
use time::Date;
use time::Weekday::*;

pub fn get_first_day_of_week(date: &Date) -> Result<Date> {
    let mut date = Date::clone(date);

    while date.weekday() != Monday {
        date = date.previous_day().context("")?;
    }

    Ok(date)
}

pub fn generate_diary_template(date: &Date) -> Result<String> {
    let mut diary_template = String::new();
    let mut date = Date::clone(date);
    let format = format_description::parse("[year]-[month]-[day]")?;

    write!(&mut diary_template, "# {}\n", &date.format(&format)?)?;
    for _ in 0..7 {
        write!(&mut diary_template, "## {}\n\n", &date.format(&format)?)?;
        date = date.next_day().context("")?;
    }

    Ok(diary_template)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

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
            "# 2021-12-27
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
