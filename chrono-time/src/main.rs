use chrono::{Datelike, Local, Timelike};

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
}
