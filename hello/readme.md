# hello 单元测试和集成测试
https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html

```rust
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
}

```

# 集成测试组织结构
在项目的根目录下新建tests目录，然后把对应的单元测试写入一个文件即可，比如：tests/add_test.rs

# 二进制 crate 的集成测试
如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测试并使用 extern crate 
导入 src/main.rs 中定义的函数。只有库 crate 才会向其他 crate 暴露了可供调用和使用的函数；二进制 crate 只意在单独运行。

这就是许多 Rust 二进制项目使用一个简单的 src/main.rs 调用 src/lib.rs 中的逻辑的原因之一。
因为通过这种结构，集成测试 就可以 通过 extern crate 测试库 crate 中的主要功能了，
而如果这些重要的功能没有问题的话，src/main.rs 中的少量代码也就会正常工作且不需要测试。

# assert使用
assert_eq!,assert!,assert_ne!等
使用 assert_eq! 和 assert_ne! 宏来测试相等

其中 assert! 宏由标准库提供，在希望确保测试中一些条件为 true 时非常有用
需要向 assert! 宏提供一个求值为布尔值的参数。如果值是 true，assert! 什么也不做，
同时测试会通过。如果值为 false，assert! 调用 panic! 宏，这会导致测试失败。
assert! 宏帮助我们检查代码是否以期望的方式运行。

```rust
// 相等比较
let y = 2 + 1;
assert_eq!(y, 3);
```

# 使用 Result<T, E> 编写测试
```rust
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
```
