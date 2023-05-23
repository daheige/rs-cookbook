use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("请输入一个数字:");
    let mut guess_number = String::new();
    stdin()
        .read_line(&mut guess_number)
        .expect("你输入的数字不合法");
    let guess: i32 = guess_number.trim().parse().expect("无效的数字");

    println!("你输入的数字是{}", guess);
    let rnd: i32 = rand::thread_rng().gen_range(1..20);
    match guess.cmp(&rnd) {
        Ordering::Less => println!("输入的数字太小了"),
        Ordering::Equal => println!("输入的数字相等"),
        Ordering::Greater => println!("数字太大了"),
    }

    println!("神秘数字是{}", rnd);
}
