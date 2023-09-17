use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    // 读取文件内容
    let result = fs::read_to_string("test.md");
    // match模式匹配
    let content = match result {
        Ok(content) => content,
        Err(err) => return Err(err.into()), // 错误提前返回
    };

    println!("文件内容: {}", content);
    Ok(())
}

// cargo test file_read -- --show-output
// 测试某个函数，并输出执行结果
//
// 这个问号 ? 是 Rust 中的错误处理操作符。
// 它用于处理 Result 类型的返回值。
// 如果 Result 是一个 Ok，则 ? 不会执行任何操作，它会将成功的值提取出来。
// 如果 Result 是一个 Err，则 ? 会立即将错误返回给调用者，作为整个函数的返回值，
// 就好像使用 return Err(...) 一样
#[test]
fn file_read() -> Result<(), Box<dyn std::error::Error>> {
    // ?错误简写模式，遇到错误就返回，不再执行下面的程序
    let content = fs::read_to_string("test.md")?;
    println!("文件内容: {}", content);
    Ok(())
}

// 自定义错误类型
#[derive(Debug)]
struct CustomError(String);

// 单元测试，读取一个不存在的文件
// 执行结果：Error: CustomError("read file err:No such file or directory (os error 2)")
#[test]
fn file_read_custom() -> Result<(), CustomError> {
    // 调用Result<T,E>类型的map_err方法，参数是op，它是一个闭包FnOnce函数
    // 其返回值是Result<T, F>，这里的F就是闭包返回的F，这个F在下面的代码中
    // 就是我们自定义的错误类型
    //
    // map_err 方法接受一个闭包（匿名函数），该闭包接受一个错误对象 err，
    // 并返回一个新的错误对象。在这个闭包中，它将原始的 std::io::Error
    // 错误转换为自定义的 CustomError 错误类型，
    // 并添加了一条包含错误信息的自定义错误消息
    let content = fs::read_to_string("test1.md")
        .map_err(|err| CustomError(format!("read file err:{}", err)))?;
    println!("read file content:{}", content);

    Ok(())
}
