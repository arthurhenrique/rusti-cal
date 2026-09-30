mod locale;

use ansi_term::{
    Color::{Black, Cyan, Purple, Red, Yellow, RGB},
    Style,
};
use chrono::{Datelike, NaiveDate};
use std::fmt::Write;

const REFORM_YEAR: u32 = 1099;
// Preserve the project's Julian leap-year rule through the reform year.
const SPECIAL_LEAP_YEARS: u64 = (REFORM_YEAR / 100 - REFORM_YEAR / 400) as u64;
const MONTHS: usize = 12;
const WEEKDAYS: u64 = 7;
const COLUMNS: usize = 3;
const ROWS: usize = MONTHS / COLUMNS;
const MONTH_LINES: usize = 8;
const WEEK_WIDTH: usize = 21;

fn is_leap_year(year: u32) -> bool {
    year.is_multiple_of(4)
        && (year <= REFORM_YEAR || !year.is_multiple_of(100) || year.is_multiple_of(400))
}

fn count_leap_years(year: u32) -> u64 {
    let previous_year = u64::from(year.saturating_sub(1));
    if year <= REFORM_YEAR {
        previous_year / 4
    } else {
        previous_year / 4 - previous_year / 100 + previous_year / 400 + SPECIAL_LEAP_YEARS
    }
}

fn days_by_year(year: u32) -> u64 {
    u64::from(year.saturating_sub(1)) * 365 + count_leap_years(year)
}

fn days_by_month(year: u32) -> [u32; MONTHS + 1] {
    let february = if is_leap_year(year) { 29 } else { 28 };
    [0, 31, february, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

fn days_by_date(
    day: u32,
    month: usize,
    year: u32,
    month_totals: [u32; MONTHS + 1],
    year_total: u64,
) -> u64 {
    u64::from(day)
        + u64::from(if month > 1 {
            month_totals[month - 1]
        } else {
            0
        })
        + if year > 1 { year_total } else { 0 }
}

fn get_days_accumulated_by_month(year: u32) -> ([u32; MONTHS + 1], [u32; MONTHS + 1]) {
    let days = days_by_month(year);
    let mut total = 0;
    let accumulated = days.map(|days| {
        total += days;
        total
    });
    (accumulated, days)
}

fn weekday_offset(ordinal: u64, starting_day: u32) -> usize {
    ((ordinal % WEEKDAYS + WEEKDAYS - 1 - u64::from(starting_day) % WEEKDAYS) % WEEKDAYS) as usize
}

struct Calendar<'a> {
    year: u32,
    starting_day: u32,
    week_numbers: bool,
    month_totals: [u32; MONTHS + 1],
    year_total: u64,
    month_names: &'a [String],
    week_names: &'a [String],
}

impl Calendar<'_> {
    fn month(&self, month: usize, days: u32) -> Vec<String> {
        let prefix = if self.week_numbers { "   " } else { "" };
        let width = WEEK_WIDTH + prefix.len();
        let mut lines = Vec::with_capacity(MONTH_LINES);
        lines.push(format!("{prefix} {:^20}", self.month_names[month - 1]));
        lines.push(format!(
            "{prefix}{}",
            circular_week_name(self.week_names, self.starting_day as usize)
        ));

        let first_day = days_by_date(1, month, self.year, self.month_totals, self.year_total);
        let offset = weekday_offset(first_day, self.starting_day);
        for week in 0..6 {
            let mut line = String::with_capacity(width);
            let first = (week * WEEKDAYS as usize).saturating_sub(offset) + 1;
            let last = ((week + 1) * WEEKDAYS as usize)
                .saturating_sub(offset)
                .min(days as usize);
            if self.week_numbers {
                let week_number = (first <= last).then_some(()).and_then(|()| {
                    i32::try_from(self.year)
                        .ok()
                        .and_then(|year| NaiveDate::from_ymd_opt(year, month as u32, last as u32))
                        .map(|date| date.iso_week().week())
                });
                if let Some(number) = week_number {
                    write!(line, "{number:>3}").expect("writing to a String cannot fail");
                } else {
                    line.push_str(prefix);
                }
            }
            for column in 0..WEEKDAYS as usize {
                let position = week * WEEKDAYS as usize + column;
                if position >= offset && position - offset < days as usize {
                    write!(line, "{:>3}", position - offset + 1)
                        .expect("writing to a String cannot fail");
                } else {
                    line.push_str("   ");
                }
            }
            lines.push(line);
        }
        lines
    }
}

fn circular_week_name(names: &[String], starting_day: usize) -> String {
    let names = names
        .iter()
        .cycle()
        .skip(starting_day % names.len())
        .take(names.len())
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");
    format!(" {names}")
}

/// Build a year as four rows of three months, each containing eight text lines.
/// Week starts are numbered from Sunday (0) and wrap every seven days.
pub fn calendar(
    year: u32,
    locale_str: &str,
    starting_day: u32,
    week_numbers: bool,
) -> Vec<Vec<Vec<String>>> {
    let locale = locale::LocaleInfo::new(locale_str);
    let month_names = locale.month_names();
    let week_names = locale.week_day_names();
    let (month_totals, months) = get_days_accumulated_by_month(year);
    let calendar = Calendar {
        year,
        starting_day: starting_day % WEEKDAYS as u32,
        week_numbers,
        month_totals,
        year_total: days_by_year(year),
        month_names: &month_names,
        week_names: &week_names,
    };
    (0..ROWS)
        .map(|row| {
            (0..COLUMNS)
                .map(|column| {
                    let month = row * COLUMNS + column + 1;
                    calendar.month(month, months[month])
                })
                .collect()
        })
        .collect()
}

fn print_row(
    row: &str,
    starting_day: u32,
    today: Option<u32>,
    monochromatic: bool,
    week_numbers: bool,
) {
    let starting_day = starting_day % WEEKDAYS as u32;
    let prefix = 3 * usize::from(week_numbers);
    let saturday = 1 + 3 * ((6 + 7 - starting_day) % 7) as usize + prefix;
    let sunday = 1 + 3 * ((7 - starting_day) % 7) as usize + prefix;
    let today = today.map(|position| 1 + 3 * position as usize + prefix);
    let row = row
        .chars()
        .enumerate()
        .map(|(index, character)| {
            let style = if today.is_some_and(|position| index == position || index == position + 1)
            {
                Black.on(RGB(200, 200, 200))
            } else if monochromatic {
                Style::new()
            } else if index == saturday || index == saturday + 1 {
                Yellow.bold()
            } else if index == sunday || index == sunday + 1 {
                Red.bold()
            } else if week_numbers && index < prefix {
                Purple.bold()
            } else {
                Style::new()
            };
            style.paint(character.to_string())
        })
        .collect::<Vec<_>>();
    print!("{} ", ansi_term::ANSIStrings(&row));
}

/// Return the month row, month column, weekday column and week row of a date.
fn get_today_position(year: u32, month: u32, day: u32, starting_day: u32) -> (u32, u32, u32, u32) {
    let (month_totals, _) = get_days_accumulated_by_month(year);
    let first = days_by_date(1, month as usize, year, month_totals, days_by_year(year));
    let position = weekday_offset(first, starting_day) as u32 + day - 1;
    (
        (month - 1) / COLUMNS as u32,
        (month - 1) % COLUMNS as u32,
        position % WEEKDAYS as u32,
        position / WEEKDAYS as u32,
    )
}

/// Print a calendar, highlighting today's date and optionally coloring weekends.
pub fn display(
    year: u32,
    locale_str: &str,
    starting_day: u32,
    monochromatic: bool,
    week_numbers: bool,
) {
    let rows = calendar(year, locale_str, starting_day, week_numbers);
    let today = chrono::Local::now();
    let today_position = (i64::from(today.year()) == i64::from(year))
        .then(|| get_today_position(year, today.month(), today.day(), starting_day));
    println!(
        "{}{}",
        " ".repeat(6 * usize::from(week_numbers)),
        Style::new().bold().paint(format!(" {year:^63}"))
    );
    for (row_index, row) in rows.iter().enumerate() {
        for line_index in 0..MONTH_LINES {
            for (column_index, month) in row.iter().enumerate() {
                let line = &month[line_index];
                if line_index == 0 {
                    let style = if monochromatic {
                        Style::new()
                    } else {
                        Cyan.bold()
                    };
                    print!("{} ", style.paint(line));
                } else {
                    let today = today_position
                        .filter(|&(row, column, _, week)| {
                            row as usize == row_index
                                && column as usize == column_index
                                && week as usize + 2 == line_index
                        })
                        .map(|(_, _, column, _)| column);
                    print_row(line, starting_day, today, monochromatic, week_numbers);
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_week_name() {
        let locale_str = "en_US";
        let locale_info = locale::LocaleInfo::new(locale_str);
        let week_name = locale_info.week_day_names();
        assert_eq!(circular_week_name(&week_name, 0), " Su Mo Tu We Th Fr Sa");
        assert_eq!(circular_week_name(&week_name, 1), " Mo Tu We Th Fr Sa Su");
        assert_eq!(circular_week_name(&week_name, 2), " Tu We Th Fr Sa Su Mo");
        assert_eq!(circular_week_name(&week_name, 3), " We Th Fr Sa Su Mo Tu");
        assert_eq!(circular_week_name(&week_name, 4), " Th Fr Sa Su Mo Tu We");
        assert_eq!(circular_week_name(&week_name, 5), " Fr Sa Su Mo Tu We Th");
        assert_eq!(circular_week_name(&week_name, 6), " Sa Su Mo Tu We Th Fr");
    }

    #[test]
    fn test_circular_week_name_pt_br() {
        let locale_str = "pt_BR";
        let locale_info = locale::LocaleInfo::new(locale_str);
        let week_name = locale_info.week_day_names();
        assert_eq!(circular_week_name(&week_name, 0), " Do Se Te Qu Qu Se Sá");
    }

    #[test]
    fn test_is_leap_year() {
        let test_cases = [
            (100, true),
            (400, true),
            (1000, true),
            (1100, false),
            (2022, false),
            (2023, false),
            (2024, true),
            (2025, false),
            (2300, false),
        ];
        for test_case in test_cases.iter() {
            assert_eq!(
                is_leap_year(test_case.0),
                test_case.1,
                "{} is {} a leap year",
                test_case.0,
                if test_case.1 { "" } else { "not" }
            );
        }
    }

    #[test]
    fn test_count_leap_years() {
        let test_cases = [(400, 99), (401, 100), (1100, 274), (1200, 298), (2022, 498)];
        for test_case in test_cases.iter() {
            assert_eq!(
                count_leap_years(test_case.0),
                test_case.1,
                "Year {}",
                test_case.0
            );
        }
    }

    #[test]
    fn test_days_by_year() {
        let test_cases = [
            (0, 0),
            (1, 0),
            (2, 365),
            (3, 730),
            (4, 1095),
            (5, 1461),
            (6, 1826),
            (7, 2191),
            (8, 2556),
            (9, 2922),
            (10, 3287),
            (400, 145734),
            (401, 146100),
            (402, 146465),
            (403, 146830),
            (404, 147195),
            (800, 291834),
            (801, 292200),
            (802, 292565),
            (803, 292930),
            (804, 293295),
            (2022, 738163),
            (2023, 738528),
            (2024, 738893),
            (2025, 739259),
        ];
        for test_case in test_cases.iter() {
            assert_eq!(
                days_by_year(test_case.0),
                test_case.1,
                "Year {}",
                test_case.0
            );
        }
    }

    #[test]
    fn test_days_by_month() {
        let not_leap = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let leap = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let test_cases = [(2022, not_leap), (2024, leap)];
        for test_case in test_cases.iter() {
            assert_eq!(
                days_by_month(test_case.0),
                test_case.1,
                "Year {}",
                test_case.0
            );
        }
    }

    #[test]
    fn test_days_by_date() {
        assert_eq!(
            days_by_date(
                0,
                0,
                0,
                [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
                1
            ),
            0
        );
        assert_eq!(
            days_by_date(
                7,
                4,
                0,
                [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
                1
            ),
            38
        );
    }

    #[test]
    fn test_get_days_accumulated_by_month() {
        assert_eq!(
            get_days_accumulated_by_month(2000),
            (
                [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
                [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            )
        );
        assert_eq!(
            get_days_accumulated_by_month(1600),
            (
                [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
                [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            )
        );
        assert_eq!(
            get_days_accumulated_by_month(1700),
            (
                [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
                [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            )
        );
    }

    #[test]
    fn calendar_has_complete_aligned_months() {
        for year in [1, 100, 1100, 2023, 2024, 9999, u32::MAX] {
            for starting_day in 0..7 {
                for week_numbers in [false, true] {
                    let rows = calendar(year, "en_US", starting_day, week_numbers);
                    assert_eq!(rows.len(), ROWS);
                    for (row_index, row) in rows.iter().enumerate() {
                        assert_eq!(row.len(), COLUMNS);
                        for (column_index, month) in row.iter().enumerate() {
                            assert_eq!(month.len(), MONTH_LINES);
                            let prefix = 3 * usize::from(week_numbers);
                            let days: Vec<u32> = month[2..]
                                .iter()
                                .flat_map(|line| {
                                    assert_eq!(line.len(), WEEK_WIDTH + prefix);
                                    line[prefix..]
                                        .split_whitespace()
                                        .map(|day| day.parse().unwrap())
                                })
                                .collect();
                            let month_index = row_index * COLUMNS + column_index + 1;
                            assert_eq!(
                                days,
                                (1..=days_by_month(year)[month_index]).collect::<Vec<_>>()
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn modern_dates_and_highlight_positions_match_chrono() {
        for year in [2023, 2024, 2025] {
            for month in 1..=12 {
                for day in 1..=days_by_month(year)[month as usize] {
                    let date = NaiveDate::from_ymd_opt(year as i32, month, day).unwrap();
                    for starting_day in 0..7 {
                        let (row, column, x, y) =
                            get_today_position(year, month, day, starting_day);
                        assert_eq!(
                            x,
                            (date.weekday().num_days_from_sunday() + 7 - starting_day) % 7
                        );
                        let calendar = calendar(year, "en_US", starting_day, true);
                        let line = &calendar[row as usize][column as usize][y as usize + 2];
                        assert_eq!(
                            line[3 + x as usize * 3..6 + x as usize * 3].trim(),
                            day.to_string()
                        );
                        let last_day = line[3..]
                            .split_whitespace()
                            .last()
                            .unwrap()
                            .parse()
                            .unwrap();
                        let last = NaiveDate::from_ymd_opt(year as i32, month, last_day).unwrap();
                        assert_eq!(line[..3].trim(), last.iso_week().week().to_string());
                    }
                }
            }
        }
    }
}
