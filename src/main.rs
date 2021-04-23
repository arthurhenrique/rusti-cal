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
        count += months_memoized[month]
    }
    if year > 1 {
        count += year_memoized
    }
    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let year = args[1].parse::<u32>().unwrap();
    let months = get_days(year);

    let m = days_by_month(year);
    let y = days_by_year(year);

    let debug: bool = false;
    if debug {
        println!("year: {}", year);
        println!("months: {:?}", get_days(year));
        println!("days_by_month: {:?}", months);
        println!("month memoization: {:?}", m);
        println!("year memoization: {}", y);
        println!("debug: {}", days_by_date(1, 1, 2020, m.clone(), y));
    }

    print!("        {}       ", year);
    for month in 1..13 {
        println!("\n\n       --{:02}--       ", month);
        println!(" Su Mo Tu We Th Fr Sa");
        for day in 1..months[month] + 1 {
            let day_year = days_by_date(1, month, year, m.clone(), y);
            if day == 1 && day_year % 7 == 0 {
                print!("                  ")
            }
            if day == 1 && day_year % 7 == 2 {
                print!("   ")
            }
            if day == 1 && day_year % 7 == 3 {
                print!("      ")
            }
            if day == 1 && day_year % 7 == 4 {
                print!("         ")
            }
            if day == 1 && day_year % 7 == 5 {
                print!("            ")
            }
            if day == 1 && day_year % 7 == 6 {
                print!("               ")
            }

            if days_by_date(day, month, year, m.clone(), y) % 7 == 1 {
                print!("{:3}", day)
            }
            if days_by_date(day, month, year, m.clone(), y) % 7 == 2 {
                print!("{:3}", day)
            }
            if days_by_date(day, month, year, m.clone(), y) % 7 == 3 {
                print!("{:3}", day)
            }
            if days_by_date(day, month, year, m.clone(), y) % 7 == 4 {
                print!("{:3}", day)
            }
            if days_by_date(day, month, year, m.clone(), y) % 7 == 5 {
                print!("{:3}", day)
            }
            if days_by_date(day, month, year, m.clone(), y) % 7 == 6 {
                print!("{:3}", day)
            }
            if days_by_date(day, month, year, m.clone(), y) % 7 == 0 {
                println!("{:3}", day)
            }
        }
    }
}
