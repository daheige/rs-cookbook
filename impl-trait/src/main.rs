// 特征区间与impl特征语法使用
// 通常建议将特征区间的 impl 特征语法用做函数的返回类型。在参数位置使用它意味着
// 我们不能使用 turbofish 运算符。如果某些相关代码使用 turbofish 运算符来调用软件包中的
// 某个方法，那么可能导致 API 不兼容。只有当我们没有可用的具体类型时才应该使用它，
// 就像闭包那样。

use std::fmt::Display;

// val 是一个impl trait约束，表明它必须实现disply trait才可以作为参数传递给函数
fn show_me(val: impl Display) {
    println!("{}", val);
}

// 通常建议将特征区间的 impl 特征语法用做函数的返回类型
// 通过impl 对函数返回值进行约束
// 下面的函数返回一个闭包Fn
fn lazy_add(a: u32, b: u32) -> impl Fn() -> u32 {
    move || a + b
}

// impl trait 作为返回值
fn surr(val: impl Display) -> impl Display {
    // 第一个{{}} 用来避免转移{}的
    format!("{{{}}}", val)
}

fn main() {
    show_me("hello,rust");

    let add_later = lazy_add(1, 2);
    println!("{:?}", add_later());

    println!("{}", surr("abc"));
}
