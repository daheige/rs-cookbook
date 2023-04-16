use std::thread;

fn main() {
    let a = "hello";
    let b = "wolrd";
    // 默认情况，闭包是借用a,b也就是说不可变引用模式&a，而不是move变量的所有权到闭包中
    let c = |msg| {
        println!("a = {},b = {},msg:{}", a, b, msg);
    };

    c("rust");

    // 通过move关键字将当前作用域的变量所有权转移到闭包中
    let s = String::from("hello world,rust");
    let handler = thread::spawn(move || {
        println!("s moved to clourse,s = {:?}", s);
    });

    handler.join().unwrap(); // 等待线程执行完毕

    // 对于非基本类型的情况下，move将所有权移动到闭包中，后续该变量就无法使用了
    let m = String::from("hello world,rust");
    let n = move || {
        println!("m = {}", m);
    };
    n();
    // println!("m:{}", m); // - variable moved due to use in closure

    // 可变变量的闭包使用情况,下面的使用是错误的用法
    // let mut s2 = String::from("hello");
    // let c3 = move || {
    //     c2 = "abc".to_string(); // cannot find value `c2` in this scope
    // };
}
