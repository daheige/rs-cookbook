use hello_derive::{HelloMacro, log_calls, make_vec};

trait HelloMacroTrait {
    fn hello();
}

// 派生宏（derive)，说明：用于像 #[derive(...)] 一样针对类型生成代码。
#[derive(HelloMacro)]
struct Pancakes;

// 使用属性宏
#[log_calls]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    Pancakes::hello();
    println!("{}", add(2, 3));
    let v = make_vec!(1, 2, 3);
    println!("{:?}", v);
}
