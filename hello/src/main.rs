fn main() {
    println!("Hello, world!");
}

// 1.和源文件代码在一起的单元测试处理 #[test] 注解下
// 测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，
// 而在运行 cargo build 时不这么做。
// 执行单元测试输出完整的过程
// 在结尾加上 --show-output 告诉 Rust 显示成功测试的输出
// cargo test -v -- --show-output
// 可以运行指定的函数 cargo test it_works -- --show-output
#[test]
fn it_works() {
    let x = 12;
    println!("hello,test");
    println!("{}", x);
}

// 2.将单元测试 放入mod tests中
// 测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，
// 而在运行 cargo build 时不这么做。这在只希望构建库的时候可以节省编译时间，并且因为它们并没有包含测试。
// 所以能减少编译产生的文件的大小。
//
// 单元测试位于与源码相同的文件中，所以你需要使用 #[cfg(test)] 来指定他们不应该被包含进编译结果中。
#[cfg(test)]
mod tests {
    //  cargo test it_works2 -- --show-output
    #[test]
    fn it_works2() {
        let x = 122;
        println!("hello,test2");
        println!("{}", x);
    }

    // 使用 Result<T, E> 编写测试
    // 现在 it_works 函数的返回值类型为 Result<(), String>。在函数体中，不同于调用 assert_eq! 宏，
    // 而是在测试通过时返回 Ok(())，在测试失败时返回带有 String 的 Err。
    //
    // 这样编写测试来返回 Result<T, E> 就可以在函数体中使用问号运算符，
    // 如此可以方便的编写任何运算符会返回 Err 成员的测试。
    //
    // 为了断言一个操作返回 Err 成员，不要使用对 Result<T, E> 值使用问号表达式（?）。
    // 而是使用 assert!(value.is_err())
    #[test]
    fn it_works3() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
