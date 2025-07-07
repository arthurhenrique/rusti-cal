mod locale;

use ansi_term::{
    Color::{Black, Cyan, Purple, Red, Yellow, RGB},
    Style,
};
use chrono::Datelike;
use unicode_width::UnicodeWidthStr;

const REFORM_YEAR: u32 = 1099;

// SPECIAL_LEAP_YEARS are years before REFORM_YEAR that are divisible by 100, but not by 400.
const SPECIAL_LEAP_YEARS: u32 = (REFORM_YEAR / 100) - (REFORM_YEAR / 400);

const MONTHS: usize = 12;
const WEEKDAYS: u32 = 7;

const COLUMN: usize = 3;
const ROWS: usize = 4;
const ROW_SIZE: usize = 7;

static TOKEN: &str = "\n";

fn is_leap_year(year: u32) -> bool {
    if year <= REFORM_YEAR {
        return year % 4 == 0;
    }
    (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
}

fn count_leap_years(year: u32) -> u32 {
    if year <= REFORM_YEAR {
        (year - 1) / 4
    } else {
        ((year - 1) / 4) - ((year - 1) / 100) + ((year - 1) / 400) + SPECIAL_LEAP_YEARS
    }
}

fn days_by_year(year: u32) -> u32 {
    if year < 1 {
        0
    } else {
        (year - 1) * 365 + count_leap_years(year)
    }
}

fn days_by_month(year: u32) -> Vec<u32> {
    let feb_day: u32 = if is_leap_year(year) { 29 } else { 28 };
    vec![0, 31, feb_day, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

fn days_by_date(
    day: u32,
    month: usize,
    year: u32,
    months_memoized: Vec<u32>,
    year_memoized: u32,
) -> u32 {
    day + (if month > 1 {
        months_memoized[month - 1]
    } else {
        0
    }) + (if year > 1 { year_memoized } else { 0 })
}

fn get_days_accumulated_by_month(year: u32) -> (Vec<u32>, Vec<u32>) {
    let days: Vec<u32> = days_by_month(year);
    let accum = days
        .iter()
        .scan(0, |acc, &x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .collect();
    (accum, days)
}

fn first_day_printable(day_year: u32, starting_day: u32) -> String {
    let mut printable = format!("");

    if (day_year - starting_day) % WEEKDAYS == 0 {
        printable.push_str("                  ");
    }
    for i in 2..WEEKDAYS {
        if (day_year - starting_day) % WEEKDAYS == i {
            printable.push_str(&"   ".repeat(i as usize - 1));
            break;
        }
    }
    printable
}

fn remain_day_printable(day: u32, day_year: u32, starting_day: u32) -> String {
    let base = if ((day_year - starting_day) % WEEKDAYS) == 0 {
        format!("{:3}{}", day, TOKEN)
    } else {
        String::default()
    };

    let complement = (1..WEEKDAYS)
        .find_map(|i| ((day_year - starting_day) % WEEKDAYS == i).then(|| format!("{:3}", day)))
        .unwrap_or_default();

    format!("{}{}", base, complement)
}

fn body_printable(
    year: u32,
    month: usize,
    days: u32,
    months_memoized: Vec<u32>,
    year_memoized: u32,
    starting_day: u32,
    week_numbers: bool,
) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let mut result_days = format!("");

    // display month formatted
    (1..days + 1).for_each(|day| {
        if day == 1 {
            let first_day = days_by_date(1, month, year, months_memoized.clone(), year_memoized);
            result_days.push_str(&first_day_printable(first_day, starting_day))
        }
        let day_year = days_by_date(day, month, year, months_memoized.clone(), year_memoized);
        result_days.push_str(&remain_day_printable(day, day_year, starting_day))
    });

    // lines splitted by '\n' TOKEN
    result_days
        .split(TOKEN)
        .collect::<Vec<&str>>()
        .into_iter()
        .for_each(|i| result.push(i.to_string()));

    for line in 0..result.len() {
        let width = UnicodeWidthStr::width(result[line].as_str());
        let spaces = 21 - width + (3 * (result[line].is_empty() && week_numbers) as usize);
        result[line] += &" ".repeat(spaces);
    }
    // all bodies should have at least 7 lines
    if result.len() < 7 {
        result.push(" ".repeat(21 + (3 * week_numbers as usize)));
    }
    result
}

fn month_printable(
    year: u32,
    month: usize,
    days: u32,
    months_memoized: Vec<u32>,
    year_memoized: u32,
    starting_day: u32,
    month_names: Vec<String>,
    week_names: Vec<String>,
    week_numbers: bool,
) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let body = body_printable(
        year,
        month,
        days,
        months_memoized,
        year_memoized,
        starting_day,
        week_numbers,
    );
    let month_name = &month_names[month - 1];
    let name_width = UnicodeWidthStr::width(month_name.as_str());
    if name_width >= 20 {
        result.push(format!(" {}", month_name));
    } else {
        let padding = 20 - name_width;
        let left = padding / 2;
        let right = padding - left;
        result.push(format!(" {}{}{}", " ".repeat(left), month_name, " ".repeat(right)));
    }
    let header = circular_week_name(week_names, starting_day as usize);
    result.push(header);

    body.into_iter().for_each(|item| {
        result.push(item);
    });
    result
}

fn circular_week_name(week_name: Vec<String>, idx: usize) -> String {
    let mut s = " ".to_string();
    for i in idx..(ROW_SIZE - 1 + idx) {
        s.push_str(&format!("{} ", week_name[i % ROW_SIZE]));
    }
    s.push_str(week_name[(ROW_SIZE - 1 + idx) % ROW_SIZE].as_str());
    s.to_string()
}

pub fn calendar(
    year: u32,
    locale_str: &str,
    starting_day: u32,
    week_numbers: bool,
) -> Vec<Vec<Vec<String>>> {
    let mut rows: Vec<Vec<Vec<String>>> = vec![vec![Vec::new(); COLUMN]; ROWS];
    let (months_memoized, months) = get_days_accumulated_by_month(year);
    let year_memoized = days_by_year(year);
    let locale_info = locale::LocaleInfo::new(locale_str);
    let month_names = locale_info.month_names();
    let week_names = locale_info.week_day_names();
    let mut week_counter = 1;

    for month in 1..=MONTHS {
        let row_idx = (month - 1) / COLUMN;
        let col_idx = (month - 1) % COLUMN;

        let mut printable = month_printable(
            year,
            month,
            months[month],
            months_memoized.clone(),
            year_memoized,
            starting_day,
            month_names.clone(),
            week_names.clone(),
            week_numbers,
        );

        if week_numbers {
            for (line_idx, line) in printable.iter_mut().enumerate() {
                if line_idx < 2 {
                    *line = format!("   {}", line);
                } else if !line.trim().is_empty() {
                    *line = format!(
                        "{}{}{}",
                        " ".repeat(1 + (week_counter < 10) as usize),
                        week_counter,
                        line
                    );

                    if line.chars().last().unwrap() != ' ' {
                        week_counter += 1;
                    }
                }
            }
        }

        rows[row_idx][col_idx] = printable;
    }

    rows
}

fn print_row(
    row: &str,
    starting_day: u32,
    today_included: bool,
    pos_today: u32,
    monochromatic: bool,
    week_numbers: bool,
) {
    let pos_saturday = (((6 - starting_day as i32) % 7) + 7) % 7 + (week_numbers as i32);
    let pos_sunday = (((7 - starting_day as i32) % 7) + 7) % 7 + (week_numbers as i32);

    let char_saturday = (1 + 3 * pos_saturday) as usize;
    let char_sunday = (1 + 3 * pos_sunday) as usize;
    let char_today = (1 + 3 * (pos_today + week_numbers as u32)) as usize;

    let row = row
        .split("")
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(i, s)| {
            if monochromatic {
                if today_included && (i == char_today || i == char_today + 1) {
                    Black.on(RGB(200, 200, 200)).paint(s)
                } else {
                    ansi_term::Style::default().paint(s)
                }
            } else {
                if today_included && (i == char_today || i == char_today + 1) {
                    Black.on(RGB(200, 200, 200)).paint(s)
                } else if i == char_saturday || i == char_saturday + 1 {
                    Yellow.bold().paint(s)
                } else if i == char_sunday || i == char_sunday + 1 {
                    Red.bold().paint(s)
                } else if week_numbers && i < 3 {
                    Purple.bold().paint(s)
                } else {
                    ansi_term::Style::default().paint(s)
                }
            }
        })
        .collect::<Vec<ansi_term::ANSIString>>();

    print!("{} ", ansi_term::ANSIStrings(&row));
}

/// calculates the positions of the given day within the overall grid
///
/// Returns a tuple
///
/// (month row, month column, day x position, line of month)
fn get_today_position(year: u32, month: u32, day: u32, starting_day: u32) -> (u32, u32, u32, u32) {
    let (months_memoized, _) = get_days_accumulated_by_month(year);

    let first_of_month = days_by_date(1, month as usize, year, months_memoized, days_by_year(year));
    let first_offset = (first_of_month - starting_day - 1) % WEEKDAYS;

    let row_index = (month - 1) / 3;
    let col_index = (month - 1) % 3;

    let absolute_pos = first_offset + day - 1;
    let x = absolute_pos % 7;
    let y = absolute_pos / 7;

    (row_index, col_index, x, y)
}

pub fn display(
    year: u32,
    locale_str: &str,
    starting_day: u32,
    monochromatic: bool,
    week_numbers: bool,
) {
    let rows = calendar(year, locale_str, starting_day, week_numbers);

    let today = {
        let now = chrono::Local::now();
        (now.year() as u32, now.month(), now.day())
    };

    let t_pos =
        (today.0 == year).then(|| get_today_position(today.0, today.1, today.2, starting_day));

    // print the year
    println!(
        "{}{}",
        " ".repeat(6 * week_numbers as usize),
        Style::new().bold().paint(format!(" {:^63}", year))
    );

    for (r, row) in rows.iter().enumerate() {
        for line in 0..8 {
            for (c, month) in row.iter().enumerate() {
                if line == 0 {
                    if monochromatic {
                        print!("{} ", &month[line]);
                    } else {
                        print!("{} ", Cyan.bold().paint(&month[line]));
                    }
                } else {
                    let (today_included, x) = t_pos
                        .filter(|p| p.0 == r as u32 && p.1 == c as u32 && p.3 + 2 == line as u32)
                        .map(|p| (true, p.2))
                        .unwrap_or((false, 0));

                    print_row(
                        &month[line],
                        starting_day,
                        today_included,
                        x,
                        monochromatic,
                        week_numbers,
                    );
                }
            }
            println!();
        }
    }
}

#[test]
fn test_circular_week_name() {
    let locale_str = "en_US";
    let locale_info = locale::LocaleInfo::new(locale_str);
    let week_name = locale_info.week_day_names();
    assert_eq!(
        circular_week_name(week_name.clone(), 0),
        " Su Mo Tu We Th Fr Sa"
    );
    assert_eq!(
        circular_week_name(week_name.clone(), 1),
        " Mo Tu We Th Fr Sa Su"
    );
    assert_eq!(
        circular_week_name(week_name.clone(), 2),
        " Tu We Th Fr Sa Su Mo"
    );
    assert_eq!(
        circular_week_name(week_name.clone(), 3),
        " We Th Fr Sa Su Mo Tu"
    );
    assert_eq!(
        circular_week_name(week_name.clone(), 4),
        " Th Fr Sa Su Mo Tu We"
    );
    assert_eq!(
        circular_week_name(week_name.clone(), 5),
        " Fr Sa Su Mo Tu We Th"
    );
    assert_eq!(
        circular_week_name(week_name.clone(), 6),
        " Sa Su Mo Tu We Th Fr"
    );
}

#[test]
fn test_circular_week_name_pt_br() {
    let locale_str = "pt_BR";
    let locale_info = locale::LocaleInfo::new(locale_str);
    let week_name = locale_info.week_day_names();
    assert_eq!(circular_week_name(week_name, 0), " Do Se Te Qu Qu Se Sá");
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
    let not_leap = vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap = vec![0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
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
            vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
            1
        ),
        0
    );
    assert_eq!(
        days_by_date(
            7,
            4,
            0,
            vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
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
            vec![0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
            vec![0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        )
    );
    assert_eq!(
        get_days_accumulated_by_month(1600),
        (
            vec![0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
            vec![0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        )
    );
    assert_eq!(
        get_days_accumulated_by_month(1700),
        (
            vec![0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
            vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        )
    );
}

#[test]
fn test_remain_day_printable() {
    assert_eq!(remain_day_printable(1, 1, 1), "  1\n");
    assert_eq!(remain_day_printable(1, 2, 1), "  1");
    assert_eq!(remain_day_printable(2, 2, 1), "  2");
    assert_eq!(remain_day_printable(31, 31, 1), " 31");
    assert_eq!(remain_day_printable(31, 31, 7), " 31");
}
