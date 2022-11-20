use rayon::prelude::*;

fn main() {
    println!("Hello, world!");
    let mut arr = [1, 0, 7, 9, 12, 13, 11];

    // rayon 为任何并行可迭代的数据类型提供 par_iter_mut 方法。
    // 这是一个类迭代器的链，可以对链内的数据并行计算
    // par_iter_mut 迭代可变类型
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);

    match_vec_member();

    find_any_member();

    // map reduce
    map_reduce();
}

// 并行测试集合中任意或所有的元素是否匹配给定断言
fn match_vec_member() {
    let mut vec = vec![24, 56, 98, 82, 4, 6, 8];
    // 任意一个元素any
    assert!(!vec.par_iter().any(|n| *n % 2 != 0));

    // all所有元素匹配
    assert!((vec.par_iter().all(|n| *n % 2 == 0)));

    vec.push(9);

    println!("{:?}", vec);
}

// 使用给定断言并行搜索项
// 使用 rayon::find_any 和 par_iter 并行搜索 vector 集合，以查找满足指定闭包中的断言的元素。
//
// 如果有多个元素满足 rayon::find_any 闭包参数中定义的断言，rayon 将返回搜索发现的第一个元素，
// 但不一定是 vector 集合的第一个元素。
// 请注意，实例中闭包的参数是对引用的引用（&&x）
fn find_any_member() {
    let v = vec![6, 2, 1, 9, 3, 8, 11];
    let f1 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f2 = v.par_iter().find_any(|&&x| x > 6);

    assert_eq!(f1, Some(&8));
    assert_eq!(f2, Some(&9));
    println!("{:?}", f1);
    println!("{:?}", f2);
}

struct Person {
    age: u32,
}

fn map_reduce() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 19 },
        Person { age: 19 },
        Person { age: 18 },
        Person { age: 17 },
        Person { age: 30 },
    ];
    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30 = v
        .par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);
    let alt_over_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();
    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_over_30 as f32 / num_over_30;
    let m = (avg_over_30 - alt_avg_over_30).abs() < f32::EPSILON;
    println!("{:?}", m);
    println!("avg over 30: {:?}", avg_over_30);
}
