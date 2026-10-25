use std::thread;

fn main() {
    let a = "hello";
    let b = "wolrd";
    // 默认情况，闭包是借用a,b也就是说不可变引用模式&a，而不是move变量的所有权到闭包中
    let c = |msg| {
        println!("a = {},b = {},msg:{}", a, b, msg);
    };

    c("rust");
    println!("current a = {},b = {}", a, b);

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

    greet();

    fn_mut_call();
}

// move 决定闭包如何捕获变量，闭包内部对变量的操作决定它能被调用多少次。
fn greet() {
    let name = "hello".to_string();
    // 下面的闭包接收参数后，返回了greeting,name
    // c通过move关键字将name的所有权已经移动到了闭包内部，所有其他地方就不能再次调用闭包了
    // c在这里属于FnOnce的闭包，只能调用一次
    let c = move |greeting: String| (greeting, name);
    let res = c("abc".to_string()); //  - name value moved here
    println!("res:{:?}", res);

    // 下面再次使用c就无法使用，编译时不通过
    // let res = c("abc".to_string()); //   ^ value used here after move
    // println!("res:{:?}", res);
    // print!("c:{:?}", c("abc".to_string())); //  - name value moved here

    // 下面的闭包c就不是FnOnce
    // 它的返回值c是一个impl Fn(String)
    // println! 不会消耗 user，只是借用它进行格式化，因此这个闭包实现了 Fn，可以多次调用
    // 另外，user 的类型是 &str，本身还是 Copy 类型；greeting:
    // String 只是每次调用时传入的新参数，与闭包能否重复调用无关。
    let user = "abc"; // 这里的user是一个Copy类型的变量，闭包c不会消耗它，字面量字符串
    let c = move |greeting: String| {
        println!("greeting: {} name:{}", greeting, user);
    };

    c("go".to_string());
    c("java".to_string());

    // 即使把 user 改成 String，只要闭包内仍然只是读取它，也依然可以多次调用：
    let user = String::from("abc");
    let c = move |greeting: String| {
        println!("greeting: {} name:{}", greeting, user);
    };

    c("go".to_string());
    c("java".to_string());

    // 下面这种情况才会变成只能调用一次的 FnOnce
    let user = String::from("abc");
    let c = move |greeting: String| {
        println!("greeting: {} name:{}", greeting, user);
        // 下面的语句会消耗user，闭包c就变成了 FnOnce 了
        let s = user;
        println!("s:{}", s);
    };

    c("go".to_string());
    // c("java".to_string()); //  - value moved into closure
}

fn fn_mut_call() {
    // FnMut类型的闭包，在闭包外部声明变量，需要采用mut定义变量为可变类型
    let mut name = String::from("hello");
    let mut name1 = String::from("wolrd");
    // 下面的闭包捕获 &mut name,&mut name1 可变引用
    let mut c1 = move || {
        name.push_str(",rust");
        println!("name:{}", name);
    };
    let mut c2 = move || {
        name1.push_str(",rust");
        println!("name1:{}", name1);
    };

    // FnMut类型的闭包可以多次调用
    call_mut(&mut c1);
    call_mut(&mut c1);
    call_mut(&mut c1);
    // 此时的c1是一个FnMut类型的闭包，可以多次调用
    c1();
    c1();

    call_mut(&mut c2);
    call_mut(&mut c2);
    c2();
    c2();

    // 下面是FnOnce调用
    call_once(c1);
    call_once(c2);

    let mut n = "abc".to_string();
    let c3 = || {
        n.push_str(",rust");
        println!("name1:{}", n);
    };

    call_twice(c3);
    println!("n:{}", n);
}

// 参数c是一个&mut impl FnMut trait
// 表明参数是一个实现了可变类型的闭包trait
fn call_mut(c: &mut impl FnMut()) {
    c();
}

// c是一个实现了FnOnce trait特征的闭包类型
// 一旦这个c闭包被调用，它就会消耗掉闭包内部的变量，闭包就不能再次调用了
fn call_once(c: impl FnOnce()) {
    c();
}

// 参数c是一个FnMut trait类型的闭包，也就是说c是一个可变引用的闭包
fn call_twice<F>(mut c: F)
where
    F: FnMut(),
{
    c();
    c();
}

fn call_fn(c: impl Fn()) {
    c();
}

fn say_hello() {
    println!("hello,world");
}

#[test]
fn test_clourse() {
    fn_mut_call();

    // Fn 是没有调用次数限制的闭包和函数，是所有 fn 函数中最高的一种
    // 基本的函数定义，它是实现了 Fn trait 的闭包类型，Fn trait 是所有闭包中最强的一个，它可以被多次调用
    let c = || {
        println!("hello");
    };
    call_fn(c);
    call_fn(c);
    // 普通函数定义，函数指针也可以作为参数，此时这里的say_hello函数就是一个实现了Fn trait的闭包类型
    call_fn(say_hello);
}
