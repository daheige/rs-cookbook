// 通过type定义类型别名
// 将一个Fn trait类型的闭包放入box中
// 由于函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的
// 函数时传递函数指针作为参数。倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函
// 数或闭包作为参数。
//
// 由于trait大小不固定，这里使用box智能指针可以将其包裹为固定大小
// 这里使用dyn表示这个Fn trait是一个动态object对象
type Thunk = Box<dyn Fn() + Send + 'static>;

// 这意味着 Kilometers 是 i32 的 同义词（synonym）
// Kilometers 并不是一个新的、单独的类型。Kilometers 类型的值将被完全当
// 作 i32 类型值来对待
type Kilometers = i32;

fn takes_long_type(f: Thunk) {
    println!("exec thunk for dyn Fn");
    f();
}

fn hello() {
    println!("hello");
}

// 这里返回一个闭包函数Fn trait，
fn returns_thunk() -> Thunk {
    Box::new(|| println!("hello,world"))
}

// 函数指针作为另一个函数的参数
fn add_one(x: i32) -> i32 {
    x + 1
}

// 这里的f是一个函数指针类型
// 。函数会被强制转换为 fn 类型（小写
// 的 f），不要与闭包 trait 的 Fn 相混淆。
// fn 被称为 函数指针（function pointer）。通过函数指
// 针允许我们使用函数作为其它函数的参数。
//
// 不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带
// 有 Fn 作为 trait bound 的泛型参数。
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

// 通过 impl Trait 语法返回闭包Fn trait
fn returns_closure() -> impl Fn(i32) -> i32 {
    // 返回闭包函数
    |x| x + 1
}

fn main() {
    println!("Hello, world!");
    unsafe {
        std::env::set_var("GO_BIN", "~/go/bin");
    }

    println!("env:{}", std::env::var("GO_BIN").unwrap());

    let f: Thunk = Box::new(|| println!("hi"));
    takes_long_type(f);

    // 将函数指针通过box作为函数的参数
    // 这里需要注意一点：
    // 函数会被强制转换为 fn 类型（小写
    // 的 f），不要与闭包 trait 的 Fn 相混淆。
    // fn 被称为 函数指针（function pointer）。通过函数指
    // 针允许我们使用函数作为其它函数的参数。
    takes_long_type(Box::new(hello));

    let f = returns_thunk();
    // 调用闭包函数
    f();

    let x = 23;
    let y: Kilometers = 12;
    println!("The value of y is: {}", y);
    // 因为 Kilometers 是 i32 的别名，它们是同一类型，可以将 i32 与 Kilometers 相加，也可以
    // 将 Kilometers 传递给获取 i32 参数的函数。但通过这种手段无法获得
    // newtype 模式所提供的类型检查的好处。
    // 换句话说，如果在某处混用 Kilometers 和 i32 的值，编译器也不会给出一个错误。
    let z = x + y;
    println!("The value of z is: {}", z);

    let a = do_twice(add_one, 10);
    println!("The value of do_twice is: {}", a);

    let v = vec![1, 2, 3];
    // collect::<Vec<String>> 这里是极速鱼写法
    // let v_strings = v.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    // 通过闭包函数作为map的回调处理
    let v_strings: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    println!("{:?}", v_strings);

    // 接收ToString trait上面的to_string方法，
    // 这里to_string方法签名是 fn to_string(&self) -> String;
    let v_strings: Vec<String> = v.iter().map(ToString::to_string).collect();
    println!("{:?}", v_strings);

    // 这里的u32表示类型，因此0u32表示u32类型的0数字
    // 我们通过 Status::Value 的初始化函数，对 map 所作用的范围内每个 u32 值创建
    // Status::Value 实例。一些人倾向于函数式风格，一些人喜欢闭包。它们会编译成相同的代码，
    // 因此，请选择对你来说更清晰的那一种。
    let list_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("list_statuses:{:?}", list_statuses);

    let x = returns_closure();
    let y = x(2); // 调用闭包函数
    println!("y = {}", y);
}
