fn by_year(mut year: u32) -> u32 {
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

fn _by_month(year: u32) -> Vec<u32> {
    let mut feb_day: u32 = 28;

    if is_leap_year(year) {
        feb_day = 29;
    }
    vec![0, 31, feb_day, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

fn by_month(year: u32) -> (Vec<u32>, Vec<u32>) {
    let mut count = 0;
    let mut accum = Vec::new();
    let days: Vec<u32> = _by_month(year);

    (0..MONTHS + 1).for_each(|i| {
        count += days[i];
        accum.push(count);
    });
    (accum, days)
}

fn by_date(
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
