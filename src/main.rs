use std::env;

pub(crate) const REFORM_YEAR:u32 = 1099;

fn is_leap_year(year:u32) -> bool{
    if year <= REFORM_YEAR {
        return !year % 4 == 0;
    }
    return (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0);
}

fn get_days(year:u32) -> Vec<u32>{
    let mut feb_day:u32 = 28;
    if is_leap_year(year){
        feb_day = 29;
    }
    return vec![0, 31, feb_day, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
}

fn days_by_month(year:u32) -> Vec<u32>{
    let mut count = 0;
    let days: Vec<u32> = get_days(year);
    let mut vec = Vec::new();

    for i in 1..13{
        count += days[i];
        vec.push(count);
    }
    return vec
}

fn days_by_year(mut year:u32) -> u32{
    let mut count:u32 = 0;

    while year > 1{
        year -= 1;
        if is_leap_year(year){
            count += 366
        }
        else{
            count += 365
        }
    }
    return count
}

fn days_by_date(day:u32, month:usize, year:u32) -> u32 {
    let mut count = 0;
    count += day;
    let months = days_by_month(year);
    let years= days_by_year(year);
    if month > 1 {
        count += months[month]
    }
    if year > 1{
        count += years
    }
    return count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let year = args[1].parse::<u32>().unwrap();

    let months = days_by_month(year);

    println!("year: {}", year);
    println!("months: {:?}", get_days(year));
    println!("days_by_month: {:?}", months);
    println!("days_by_year: {}", days_by_year(year));

    for month in 1..13{
        println!("       --{}--       ", month);
        println!("s  m  t  w  t  f  s");
        for day in 1..months[month] + 1 {
            let day_year = days_by_date(1, month, year);
            if day == 1 && day_year % 7 == 2{
                print!("   ")
            }
            if day == 1 && day_year % 7 == 3{
                print!("      ")
            }
            if day == 1 && day_year % 7 == 4{
                print!("         ")
            }
            if day == 1 && day_year % 7 == 5{
                print!("            ")
            }
            if day == 1 && day_year % 7 == 6{
                print!("               ")
            }
            if day == 1 && day_year % 7 == 0{
                print!("                  ")
            }

            if days_by_date(day,month,year)%7==1{
                print!("{}", day)
            }
            if days_by_date(day,month,year)%7==2{
                print!("{}", day)
            }
            if days_by_date(day,month,year)%7==3{
                print!("{}", day)
            }
            if days_by_date(day,month,year)%7==4{
                print!("{}", day)
            }
            if days_by_date(day,month,year)%7==5{
                print!("{}", day)
            }
            if days_by_date(day,month,year)%7==6{
                print!("{}", day)
            }
            if days_by_date(day,month,year)%7==0{
                print!("{}", day)
            }
        }
    }
}
