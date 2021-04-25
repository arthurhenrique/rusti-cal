use std::env;

pub(crate) const REFORM_YEAR: u32 = 1099;
pub(crate) const COLUMN: usize = 3;
pub(crate) const ROWS: usize = 4;
static REPLACE_TOKEN: &'static str = "\n";

fn is_leap_year(year: u32) -> bool {
    if year <= REFORM_YEAR {
        return year % 4 == 0;
    }
    return (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0);
}

fn get_days(year: u32) -> Vec<u32> {
    let mut feb_day: u32 = 28;
    if is_leap_year(year) {
        feb_day = 29;
    }
    return vec![0, 31, feb_day, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
}

fn days_by_month(year: u32) -> (Vec<u32>, Vec<u32>) {
    let mut count = 0;
    let days: Vec<u32> = get_days(year);
    let mut result = Vec::new();

    for i in 0..13 {
        count += days[i];
        result.push(count);
    }
    return (result, days);
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

fn print_first_day(day_year: u32) -> String {
    let mut spaces: String = "".to_string();
    let mut printable = format!("");

    if day_year % 7 == 0 {
        printable += &format!("                  ");
    }
    for i in 2..7 {
        spaces += &"   ".to_string();
        if day_year % 7 == i {
            printable += &format!("{}", spaces);
        }
    }

    return printable;
}

fn print_remain_day(day: u32, day_year: u32) -> String {
    let mut printable = format!("");

    if day_year % 7 == 0 {
        printable += &format!("{:3}{}", day, REPLACE_TOKEN)
    }
    for i in 1..7 {
        if day_year % 7 == i {
            printable += &format!("{:3}", day);
        }
    }

    return printable;
}

fn calendar(year: u32, months: Vec<u32>, memoized_month: Vec<u32>, memoized_year: u32) -> Vec<Vec<String>> {
    let width = COLUMN;
    let height = ROWS;
    let mut rows: Vec<Vec<String>> = vec![vec![String::from(""); width]; height];

    let mut row_counter = 0;
    let mut month_printable = format!("");

    for month in 1..13 {
        month_printable += &format!("        --{:02}--        {}", month, REPLACE_TOKEN);
        month_printable += &format!(" Su Mo Tu We Th Fr Sa{}", REPLACE_TOKEN);
        for day in 1..months[month] + 1 {
            if day == 1 {
                let first_day = days_by_date(1, month, year, memoized_month.clone(), memoized_year);
                month_printable += &print_first_day(first_day)
            }
            let day_year = days_by_date(day, month, year, memoized_month.clone(), memoized_year);
            month_printable += &print_remain_day(day, day_year)
        }
        // columns splited
        rows[row_counter][(month - 1) % COLUMN] = month_printable;
        if month % COLUMN == 0 {
            row_counter += 1;
        }
        month_printable = (&"").to_string();
    }

    return rows;
}

fn display(year: u32, rows: Vec<Vec<String>>) {
    println!("         {}         ", year);
    for row in rows {
        for column in row {
            println!("{}", column)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let year = args[1].parse::<u32>().unwrap();

    let (memoized_month, months) = days_by_month(year);
    let memoized_year = days_by_year(year);
    let rows = calendar(year, months,  memoized_month, memoized_year);

    display(year, rows)
}
