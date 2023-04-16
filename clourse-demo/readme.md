# clourse闭包的本质
- 闭包是一种匿名类型，一旦声明，就会产生一个新的类型，但这个类型无法被其它地方使用。这个类型就像一个结构体，会包含所有捕获的变量。
- 闭包是将函数，或者说代码和其环境一起存储的一种数据结构。(闭包也是一种数据结构吗？) 闭包引用的上下文中的自由变量，会被捕获到闭包的结构中，成为闭包类型的一部分。

# rust语言中的闭包
- 闭包会根据内部的使用情况，捕获环境中的自由变量
- 默认情况下，不带move时，闭包捕获闭包外部变量的不可变引用，也就是借用机制
- 如果要获取变量的所有权，就需要用move 关键字，转移变量的所有权ownership到闭包中
- 对于非基本类型的情况下，move将所有权移动到闭包中，后续该变量就无法使用了

```rust
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
```
