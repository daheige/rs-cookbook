// 对外提供add函数供c语言调用
#[unsafe(no_mangle)]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_rust() {
    println!("called call_from_rust!");
}

// 关闭混淆功能以让c程序可以找到调用的函数
// extern 默认导出为C ABI
#[unsafe(no_mangle)]
pub extern "C" fn say_hello(){
    println!("call func from rust");
    println!("hello,{}","world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}