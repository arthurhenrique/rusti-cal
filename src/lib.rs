const REFORM_YEAR: u32 = 1099;
const MONTHS: usize = 12;
const WEEKDAYS: u32 = 7;

const COLUMN: usize = 3;
const ROWS: usize = 4;

static TOKEN: &'static str = "\n";

fn is_leap_year(year: u32) -> bool {
    if year <= REFORM_YEAR {
        return year % 4 == 0;
    }
    (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
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
    count
}

fn days_by_month(year: u32) -> Vec<u32> {
    let mut feb_day: u32 = 28;

    if is_leap_year(year) {
        feb_day = 29;
    }
    vec![0, 31, feb_day, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
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
    count
}

fn get_days_accumulated_by_month(year: u32) -> (Vec<u32>, Vec<u32>) {
    let mut count = 0;
    let mut accum = Vec::new();
    let days: Vec<u32> = days_by_month(year);

    (0..MONTHS + 1).for_each(|i| {
        count += days[i];
        accum.push(count);
    });
    (accum, days)
}

fn first_day_printable(day_year: u32) -> String {
    let mut spaces: String = "".to_string();
    let mut printable = format!("");

    if day_year % WEEKDAYS == 0 {
        printable.push_str(&format!("                  "));
    }
    for i in 2..WEEKDAYS {
        spaces += &"   ".to_string();
        if day_year % WEEKDAYS == i {
            printable.push_str(&format!("{}", spaces));
            break;
        }
    }
    printable
}

fn remain_day_printable(day: u32, day_year: u32) -> String {
    let mut printable = format!("");

    if day_year % WEEKDAYS == 0 {
        printable.push_str(&format!("{:3}{}", day, TOKEN))
    }
    for i in 1..WEEKDAYS {
        if day_year % WEEKDAYS == i {
            printable.push_str(&format!("{:3}", day));
            break;
        }
    }
    printable
}

fn body_printable(
    year: u32,
    month: usize,
    days: u32,
    months_memoized: Vec<u32>,
    year_memoized: u32,
) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let mut result_days = format!("");

    (1..days + 1).for_each(|day| {
        if day == 1 {
            let first_day = days_by_date(1, month, year, months_memoized.clone(), year_memoized);
            result_days.push_str(&first_day_printable(first_day))
        }
        let day_year = days_by_date(day, month, year, months_memoized.clone(), year_memoized);
        result_days.push_str(&remain_day_printable(day, day_year))
    });

    result_days
        .split(TOKEN)
        .collect::<Vec<&str>>()
        .into_iter()
        .for_each(|i| result.push(i.to_string()));
    result
}

fn month_printable(
    year: u32,
    month: usize,
    days: u32,
    months_memoized: Vec<u32>,
    year_memoized: u32,
) -> Vec<String> {
    let mut result = Vec::<String>::new();
    // header
    result.push(format!("        --{:02}--        ", month));
    result.push(format!(" Su Mo Tu We Th Fr Sa"));

    // body
    let body = body_printable(year, month, days, months_memoized, year_memoized);
    for item in body {
        result.push(item.to_string());
    }
    result
}

pub fn calendar(year: u32) -> Vec<Vec<Vec<String>>> {
    let mut rows: Vec<Vec<Vec<String>>> = vec![vec![vec![String::from("")]; COLUMN]; ROWS];
    let mut row_counter = 0;

    let (months_memoized, months) = get_days_accumulated_by_month(year);
    let year_memoized = days_by_year(year);

    (1..MONTHS + 1).for_each(|month| {
        let result = month_printable(
            year,
            month,
            months[month],
            months_memoized.clone(),
            year_memoized.clone(),
        );
        rows[row_counter][(month - 1) % COLUMN] = result.clone();
        // columns splited
        if month % COLUMN == 0 {
            row_counter += 1;
        }
    });
    rows
}

pub fn display(year: u32, rows: Vec<Vec<Vec<String>>>) {
    println!("         {}         ", year);
    for row in rows {
        for column in row {
            for item in column {
                println!("{}", item)
            }
        }
    }
}
