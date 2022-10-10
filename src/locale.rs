use pure_rust_locales::Locale;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub struct LocaleInfo {
    locale: Locale,
}

impl LocaleInfo {
    pub fn new(locale_str: &str) -> LocaleInfo {
        let locale: Locale = match locale_str.try_into() {
            Ok(l) => l,
            _ => "POSIX".try_into().unwrap(),
        };
        LocaleInfo { locale }
    }

    pub fn month_names(&self) -> Vec<String> {
        let months = pure_rust_locales::locale_match!(self.locale => LC_TIME::MON);
        months.iter().map(|month| to_titlecase(month)).collect()
    }

    pub fn week_day_names(&self) -> Vec<String> {
        let abbreviated_days = pure_rust_locales::locale_match!(self.locale => LC_TIME::ABDAY);
        abbreviated_days
            .iter()
            .map(|day| to_titlecase(day))
            .map(|day| match day.chars().count() {
                1 => format!("{} ", day),
                _ => day.chars().take(2).collect(),
            })
            .collect()
    }
}

fn to_titlecase(str: &str) -> String {
    str.chars()
        .enumerate()
        .map(|(pos, c)| {
            if pos == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[test]
fn parse_invalid_locale() {
    assert_eq!(LocaleInfo::new(" ").locale, Locale::POSIX);
    assert_eq!(LocaleInfo::new("bogus").locale, Locale::POSIX);
}

#[test]
fn parse_default_locale() {
    let res = LocaleInfo::new("");
    assert_eq!(res.locale, Locale::POSIX);

    let months = res.month_names();
    assert_eq!(months.len(), 12);
    assert_eq!(months[0], "January");
    assert_eq!(months[1], "February");
    assert_eq!(months[2], "March");
    assert_eq!(months[3], "April");
    assert_eq!(months[4], "May");
    assert_eq!(months[5], "June");
    assert_eq!(months[6], "July");
    assert_eq!(months[7], "August");
    assert_eq!(months[8], "September");
    assert_eq!(months[9], "October");
    assert_eq!(months[10], "November");
    assert_eq!(months[11], "December");

    let days = res.week_day_names();
    assert_eq!(days, ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]);
}

#[test]
fn parse_english_locale() {
    let res = LocaleInfo::new("en_AU");
    assert_eq!(res.locale, Locale::en_AU);

    let months = res.month_names();
    assert_eq!(months.len(), 12);
    assert_eq!(months[0], "January");
    assert_eq!(months[1], "February");
    assert_eq!(months[2], "March");
    assert_eq!(months[3], "April");
    assert_eq!(months[4], "May");
    assert_eq!(months[5], "June");
    assert_eq!(months[6], "July");
    assert_eq!(months[7], "August");
    assert_eq!(months[8], "September");
    assert_eq!(months[9], "October");
    assert_eq!(months[10], "November");
    assert_eq!(months[11], "December");

    let days = res.week_day_names();
    assert_eq!(days, ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"]);
}

#[test]
fn parse_non_english_locale() {
    let res = LocaleInfo::new("hu_HU");
    assert_eq!(res.locale, Locale::hu_HU);

    let months = res.month_names();
    assert_eq!(months.len(), 12);
    assert_eq!(months[0], "Január");
    assert_eq!(months[1], "Február");
    assert_eq!(months[2], "Március");
    assert_eq!(months[3], "Április");
    assert_eq!(months[4], "Május");
    assert_eq!(months[5], "Június");
    assert_eq!(months[6], "Július");
    assert_eq!(months[7], "Augusztus");
    assert_eq!(months[8], "Szeptember");
    assert_eq!(months[9], "Október");
    assert_eq!(months[10], "November");
    assert_eq!(months[11], "December");

    let days = res.week_day_names();
    assert_eq!(days, ["V ", "H ", "K ", "Sz", "Cs", "P ", "Sz"]);
}

#[test]
fn test_titlecase() {
    assert_eq!(to_titlecase("January"), "January");
    assert_eq!(to_titlecase("április"), "Április");
}
