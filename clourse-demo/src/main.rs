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

    greet();

    fn_mut_call();
}

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

    // 下面的闭包c就不是FnOnce
    let user = "abc";
    let c = move |greeting: String| {
        println!("greeting: {} name:{}", greeting, user);
    };

    c("go".to_string());
    c("java".to_string());
}

fn fn_mut_call() {
    // 声明的时候是需要采用mut定义变量为可变类型
    let mut name = String::from("hello");
    let mut name1 = String::from("wolrd");
    // 下面的闭包捕获 &mut name,&mut name1
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

    call_mut(&mut c2);
    call_mut(&mut c2);

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
// 表明参数是一个可变类型的闭包
fn call_mut(c: &mut impl FnMut()) {
    c();
}

// c是一个实现了FnOnce trait特征的闭包类型
fn call_once(c: impl FnOnce()) {
    c();
}

// c是一个FnMut特征
fn call_twice<F>(mut c: F)
where
    F: FnMut(),
{
    c();
    c();
    c();
}
