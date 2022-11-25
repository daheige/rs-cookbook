use std::thread;
use std::time::{Duration, Instant};

// chrono库
use chrono::{DateTime, Duration as cDuration, FixedOffset, Local, Utc};

fn day_earlier(date_time: DateTime<Local>) -> Option<DateTime<Local>> {
    date_time.checked_sub_signed(cDuration::days(1))
}

fn main() {
    let start = Instant::now();
    // ...code...
    thread::sleep(Duration::from_millis(600));
    let duration = start.elapsed(); // 时间间隔
    println!("time elapsed: {:?}", duration); // time elapsed: 602.275921ms

    let now = Local::now();
    println!("now:{:?}", now.to_string());

    // 使用 DateTime::checked_add_signed 计算并显示两周之后的日期和时间，
    // 使用 DateTime::checked_sub_signed 计算并显示前一天的日期。
    // 如果无法计算出日期和时间，这些方法将返回 None。
    let t = now
        .checked_add_signed(cDuration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(cDuration::weeks(1)))
        .and_then(day_earlier);
    match t {
        Some(x) => println!("x:{}", x),
        None => println!("can not use chron to tell me time"),
    }

    // 时间的时区转换
    // 使用 offset::Local::now 获取本地时间并显示，然后使用 DateTime::from_utc
    // 结构体方法将其转换为 UTC 标准格式。最后，使用 offset::FixedOffset 结构体，
    // 可以将 UTC 时间转换为 UTC+8 和 UTC-2。
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let china_timezone = FixedOffset::east_opt(8 * 3600).unwrap();
    let rio_timezone = FixedOffset::west_opt(2 * 3600).unwrap();
    println!("local time:{:?}", local_time);
    println!("utc time:{:?}", utc_time);
    println!(
        "time in china now:{}",
        utc_time.with_timezone(&china_timezone)
    );
    println!("time in rio now:{}", utc_time.with_timezone(&rio_timezone));
}
