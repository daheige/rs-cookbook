use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

fn main() {
    let now = Local::now();
    let (is_pm, hour) = now.hour12();
    println!(
        "the current local time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "pm" } else { "am" }
    );

    println!(
        "and there have been {} seconds since midnight",
        now.num_seconds_from_midnight()
    );

    let (is_common_era, year) = now.year_ce();
    println!(
        "the current local date is: {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "ce" } else { "bce" }
    );
    println!(
        "and the common era began {} days ago",
        now.num_days_from_ce()
    );

    println!("current time:{}", now.format("%Y-%m-%d %H:%M:%S"));

    // 要解析不带时区的日期和时间，请使用 NaiveDate、NaiveTime，以及 NaiveDateTime
    let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S");
    println!("{}", no_timezone.unwrap());

    let date_only = NaiveDate::parse_from_str("2022-11-23", "%Y-%m-%d").unwrap();
    println!("date:{}", date_only);

    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S").unwrap();
    println!("{}", time_only);
}
