use colored::*; // 用于颜色输出
use rand::RngExt;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let mut rng = rand::rng();
    let rnd: i32 = rng.random_range(1..10); // 生成一个1到10之间的随机数
    loop {
        println!("请输入一个数字:");
        let mut guess_number = String::new();
        stdin()
            .read_line(&mut guess_number)
            .expect("你输入的字符串非法");
        let guess: i32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("err:{}", err);
                continue;
            }
        };

        println!("你输入的数字是{}", guess);

        match guess.cmp(&rnd) {
            Ordering::Less => println!("{}", "输入的数字太小了".red()),
            Ordering::Equal => {
                println!("{}", "输入的数字相等".green());
                break;
            }
            Ordering::Greater => println!("{}", "数字太大了".yellow()),
        }
    }
}
