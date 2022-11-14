// 使用 lazy_static 声明全局状态
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    // 生命一个static 静态可变变量
    // mutex保护变量，可跨多个线程处理
    static ref FRUIT:Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert_data(fruit: &str) -> Result<(), &'static str> {
    let mut db = FRUIT.lock().map_err(|_| "failed to acquire mutex guard")?;
    db.push(fruit.to_string());
    Ok(())
}

fn main() -> Result<(), &'static str> {
    println!("Hello, world!");
    insert_data("apple")?;
    insert_data("orange")?;
    insert_data("peach")?;
    {
        // 块级作用范围
        let db = FRUIT.lock().map_err(|_| "failed to acquire mutex lock")?;
        db.iter().enumerate().for_each(|(i, item)| {
            println!("index:{} item:{}", i, item);
        });
    }

    insert_data("grape")?;
    Ok(())
}
