


fn circular_week_name(week_name: Vec<&str>, idx: usize) -> String {
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
    result.push(format!("{}", MONTH_NAMES[month - 1]));
    
    let header = circular_week_name(week_name, starting_day as usize);
    result.push(header);

    body.into_iter().for_each(|item| {
        result.push(item.to_string());
    });
    result
}

fn print_row_columns(year: u32, rows: Vec<Vec<Vec<String>>>) {
    println!("                                {}", year);
    for row in rows {
        for i in 0..8 {
            for j in 0..3 {
                print!("{} ", &row[j][i]);
            }
            println!();
        }
    }
}