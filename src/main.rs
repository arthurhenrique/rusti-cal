use std::env;

pub(crate) const REFORM_YEAR: u32 = 1099;
pub(crate) const MONTHS: usize = 12;
pub(crate) const WEEKDAYS: u32 = 7;

pub(crate) const COLUMN: usize = 3;
pub(crate) const ROWS: usize = 4;

static TOKEN: &'static str = "\n";

fn is_leap_year(year: u32) -> bool {
    if year <= REFORM_YEAR {
        return year % 4 == 0;
    }
    return (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0);
}

fn days_by_year(mut year: u32) -> u32 {
    let mut count: u32 = 0;

    while year > 1 {
        year -= 1;
        if is_leap_year(year) {
            count += 366
        } else {
            count += 365
        }
    }
    return count;
}

fn days_by_month(year: u32) -> Vec<u32> {
    let mut feb_day: u32 = 28;

    if is_leap_year(year) {
        feb_day = 29;
    }
    return vec![0, 31, feb_day, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
}

fn days_by_date(
    day: u32,
    month: usize,
    year: u32,
    months_memoized: Vec<u32>,
    year_memoized: u32,
) -> u32 {
    let mut count = 0;

    count += day;
    if month > 1 {
        count += months_memoized[month - 1]
    }
    if year > 1 {
        count += year_memoized
    }
    return count;
}

fn get_days_accumulated_by_month(year: u32) -> (Vec<u32>, Vec<u32>) {
    let mut count = 0;
    let mut acc = Vec::new();
    let days: Vec<u32> = days_by_month(year);

    (0..MONTHS + 1).for_each(|i| {
        count += days[i];
        acc.push(count);
    });
    return (acc, days);
}

fn first_day_printable(day_year: u32) -> String {
    let mut spaces: String = "".to_string();
    let mut printable = format!("");

    if day_year % WEEKDAYS == 0 {
        printable += &format!("                  ");
    }
    (2..WEEKDAYS).for_each(|i| {
        spaces += &"   ".to_string();
        if day_year % WEEKDAYS == i {
            printable += &format!("{}", spaces);
        }
    });
    return printable;
}

fn remain_day_printable(day: u32, day_year: u32) -> String {
    let mut printable = format!("");

    if day_year % WEEKDAYS == 0 {
        printable += &format!("{:3}{}", day, TOKEN)
    }
    (1..WEEKDAYS).for_each(|i| {
        if day_year % WEEKDAYS == i {
            printable += &format!("{:3}", day);
        }
    });
    return printable;
}

fn month_printable(
    year: u32,
    month: usize,
    days: u32,
    months_memoized: Vec<u32>,
    year_memoized: u32,
) -> String {
    let mut result = format!("");

    result += &format!("        --{:02}--        {}", month, TOKEN);
    result += &format!(" Su Mo Tu We Th Fr Sa{}", TOKEN);

    (1..days + 1).for_each(|day| {
        if day == 1 {
            let first_day = days_by_date(1, month, year, months_memoized.clone(), year_memoized);
            result += &first_day_printable(first_day)
        }
        let day_year = days_by_date(day, month, year, months_memoized.clone(), year_memoized);
        result += &remain_day_printable(day, day_year)
    });
    return result;
}

fn calendar(year: u32) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = vec![vec![String::from(""); COLUMN]; ROWS];
    let mut row_counter = 0;

    let (months_memoized, months) = get_days_accumulated_by_month(year);
    let year_memoized = days_by_year(year);

    (1..MONTHS + 1).for_each(|month| {
        rows[row_counter][(month - 1) % COLUMN] = month_printable(
            year,
            month,
            months[month],
            months_memoized.clone(),
            year_memoized,
        );

        // columns splited
        if month % COLUMN == 0 {
            row_counter += 1;
        }
    });
    return rows;
}

fn display(year: u32, rows: Vec<Vec<String>>) {
    println!("         {}         ", year);
    rows.into_iter().for_each(|row| {
        row.into_iter().for_each(|column| println!("{}", column));
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let year = args[1].parse::<u32>().unwrap();
    display(year, calendar(year))
}
