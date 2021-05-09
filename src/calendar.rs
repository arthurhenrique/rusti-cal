

fn is_leap_year(year: u32) -> bool {
    if year <= REFORM_YEAR {
        return year % 4 == 0;
    }
    (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
}


fn calendar(year: u32, starting_day: u32) -> Vec<Vec<Vec<String>>> {
    let mut rows: Vec<Vec<Vec<String>>> = vec![vec![vec![String::from("")]; COLUMN]; ROWS];
    let mut row_counter = 0;
    let mut column_counter = 0;
    let (months_memoized, months) = get_days_accumulated_by_month(year);
    let year_memoized = days_by_year(year);

    (1..MONTHS + 1).for_each(|month| {
        rows[row_counter][column_counter] = month_printable(
            year,
            month,
            months[month],
            months_memoized.clone(),
            year_memoized.clone(),
            starting_day,
        );
        column_counter = month % COLUMN;
        if column_counter == 0 {
            row_counter += 1;
        }
    });
    rows
}

pub fn display(year: u32, starting_day: u32) {
    // io::play
    // handler_calendar
}
