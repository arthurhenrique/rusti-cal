use std::env;

pub(crate) const REFORM_YEAR: u32 = 1099;

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

fn days_by_month(year: u32) -> Vec<u32> {
    let mut count = 0;
    let days: Vec<u32> = get_days(year);
    let mut result = Vec::new();

    for i in 0..13 {
        count += days[i];
        result.push(count);
    }
    return result;
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
        printable += &format!("{:3}\n", day)
    }
    for i in 1..7 {
        if day_year % 7 == i {
            printable += &format!("{:3}", day);
        }
    }

    return printable;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let year = args[1].parse::<u32>().unwrap();
    let months = get_days(year);

    // data memoized
    let m = days_by_month(year);
    let y = days_by_year(year);

    // row x column controller
    let width = 3;
    let height = 4;
    let mut rows: Vec<Vec<String>> = vec![vec![String::from(""); width]; height];
    let mut row_counter = 0;
    let mut mod_idx;

    // display
    println!("         {}", year);
    let mut month_printable = format!("");
    for month in 1..13 {
        month_printable += &format!("\n        --{:02}--\n", month);
        month_printable += &format!(" Su Mo Tu We Th Fr Sa\n");
        for day in 1..months[month] + 1 {
            if day == 1 {
                let first_day = days_by_date(1, month, year, m.clone(), y);
                month_printable += &print_first_day(first_day)
            }
            let day_year = days_by_date(day, month, year, m.clone(), y);
            month_printable += &print_remain_day(day, day_year)
        }

        mod_idx = (month - 1) % 3;
        rows[row_counter][mod_idx] = month_printable;
        if month % 3 == 0 {
            row_counter += 1;
        }
        month_printable = (&"").to_string();
    }

    for row in rows {
        for column in row {
            print!("{}", column)
        }
    }
}
