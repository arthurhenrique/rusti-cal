mod locale;

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

fn first_day_printable(day_year: u32, starting_day: u32) -> String {
    let mut spaces: String = "".to_string();
    let mut printable = format!("");

    if (day_year - starting_day) % WEEKDAYS == 0 {
        printable.push_str(&format!("                  "));
    }
    for i in 2..WEEKDAYS {
        spaces += &"   ".to_string();
        if (day_year - starting_day) % WEEKDAYS == i {
            printable.push_str(&format!("{}", spaces));
            break;
        }
    }
    printable
}

fn remain_day_printable(day: u32, day_year: u32, starting_day: u32) -> String {
    let mut printable = format!("");

    if (day_year - starting_day) % WEEKDAYS == 0 {
        printable.push_str(&format!("{:3}{}", day, TOKEN))
    }
    for i in 1..WEEKDAYS {
        if (day_year - starting_day) % WEEKDAYS == i {
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
    starting_day: u32,
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

    // all body should have at least 6 lines
    let len = result.len();
    if len <= 6 {
        let spaces = 21 - result[len - 1].len();
        if result[len - 1].len() < 20 {
            for _i in 0..spaces {
                result[len - 1] += " "
            }
        }
        result.push("                     ".to_string())
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
) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let body = body_printable(
        year,
        month,
        days,
        months_memoized,
        year_memoized,
        starting_day,
    );
    let month_name = &month_names[month - 1];
    result.push(format!(" {:^20}", month_name));
    let header = circular_week_name(week_names, starting_day as usize);
    result.push(header);

    body.into_iter().for_each(|item| {
        result.push(item.to_string());
    });
    result
}

fn circular_week_name(week_name: Vec<String>, idx: usize) -> String {
    let mut s = format!(" ");
    let mut i = idx;

    while i < 7 + idx {
        if i == 6 + idx {
            s.push_str(&format!("{}", week_name[i % 7]));
        } else {
            s.push_str(&format!("{} ", week_name[i % 7]));
        }
        i += 1
    }
    s.to_string()
}

pub fn calendar(year: u32, locale_str: &str, starting_day: u32) -> Vec<Vec<Vec<String>>> {
    let mut rows: Vec<Vec<Vec<String>>> = vec![vec![vec![String::from("")]; COLUMN]; ROWS];
    let mut row_counter = 0;
    let mut column_counter = 0;
    let (months_memoized, months) = get_days_accumulated_by_month(year);
    let year_memoized = days_by_year(year);
    let locale_info = locale::LocaleInfo::new(locale_str);

    (1..MONTHS + 1).for_each(|month| {
        rows[row_counter][column_counter] = month_printable(
            year,
            month,
            months[month],
            months_memoized.clone(),
            year_memoized.clone(),
            starting_day,
            locale_info.month_names(),
            locale_info.week_day_names(),
        );
        column_counter = month % COLUMN;
        if column_counter == 0 {
            row_counter += 1;
        }
    });
    rows
}

pub fn display(year: u32, locale_str: &str, starting_day: u32) {
    let rows = calendar(year, locale_str, starting_day);
    println!(" {:^63}", year);
    for row in rows {
        for i in 0..8 {
            for j in 0..3 {
                print!("{} ", &row[j][i]);
            }
            println!();
        }
    }
}
