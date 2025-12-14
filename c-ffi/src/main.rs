// 使用extern函数调用外部代码
unsafe extern "C"{
    fn abs(input: i32) -> i32;

    // 将sqrt标记为安全的函数
    // safe fn sqrt(input: f64) -> f64;
    // let c = sqrt(144.0); // 这里可以不用unsafe包裹

    fn sqrt(input: f64) -> f64;
}

// 从其他语言调用rust函数
// 也可以使用 extern 来创建一个允许其它语言调用 Rust 函数的接口。不同于创建整个
// extern 块，就在 fn 关键字之前增加 extern 关键字并为相关函数指定所用到的 ABI。
// 还需增加 #[no_mangle] 注解来告诉 Rust 编译器不要 mangle 此函数的名称。
// Mangling 指编译器将我们命名的函数名更改为包含更多供其他编译过程使用的信息
// 的名称，不过可读性较差。每一个编程语言的编译器都会以稍微不同的方式 mangle
// 函数名，所以为了使 Rust 函数能在其他语言中指定，必须禁用 Rust 编译器的 name
// mangling。这是不安全的因为在没有内置 mangling 的时候在库之间可能有命名冲
// 突，所以确保所选的名称可以不用 mangling 地安全导出是我们的责任。
// #[unsafe(no_mangle)]
// pub extern "C" fn call_from_c() {
//  println!("Just called a Rust function from C!");
// }

// 定义静态变量
// 静态变量智能存储拥有'static生命周期的引用，rust可自动推导，不需要显式标注，它有一个固定的内存地址
static HELLO_WORLD: &str = "Hello, World!";

// 定义可变的静态变量
static mut COUNTER: u32 = 0;

// 访问和修改静态变量需要用unsafe包裹
unsafe fn add_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("Hello, world!");

    // 通过unsafe块包裹，调用c语言的函数
    unsafe {
        println!("abs(-2) = {}", abs(-2));
        println!("abs(-3) = {}", abs(-3));
    }

    println!("{}", HELLO_WORLD);
    unsafe {
        add_count(1);
        // 通过原始指针方式访问，并解引用方式访问
        println!("count: {}", *(&raw const COUNTER));
    }


    unsafe {
        let c = sqrt(144.0);
        println!("c = {}", c);
    }

    // 创建一个不可变裸指针和一个可变裸指针
    // 为何还要使用裸指针呢？一个主要的应用场景便是调用 C 代码接口
    let mut num = 12;
    // &raw const num 会创建一个 *const i32 的不可变裸指针。
    let r1 = &raw const num;
    let r2 = &raw mut num; // 可变的裸指针
    // 可以在安全代码中创建裸指针，但不能 解引用 裸指针和读取其指向的数据
    // println!("r1 = {}", *r1); // ^^^ dereference of raw pointer
    // println!("r2 = {}", *r2);
    unsafe {
        // 通过unsafe包裹，然后解引用访问值
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }
}
