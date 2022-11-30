// 使用 lazy_static 声明全局状态
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    // 生命一个static 静态可变变量
    // mutex保护变量，可跨多个线程处理
    static ref FRUIT:Mutex<Vec<String>> = Mutex::new(Vec::new());
}

lazy_static! {
    // 这里不需要对map进行lock，因为后面的程序只有读取操作
    static ref PRIVILEDGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("james", vec!["user", "admin"]);
        map.insert("jim", vec!["user"]);
        map
    };
}

fn show_access(name: &str) {
    let access = PRIVILEDGES.get(name);
    println!("{}:{:?}", name, access);
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

    let access = PRIVILEDGES.get("james");
    println!("james:{:?}", access);
    show_access("jim");

    Ok(())
}
