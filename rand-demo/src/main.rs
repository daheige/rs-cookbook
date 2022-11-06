use rand::{thread_rng, Rng};

// 用于生成随机字符串0-9,a-z,A-Z组成的随机字符串
use rand::distributions::Alphanumeric;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    // 生成(0,1]的浮点数
    println!("random f64: {}", rng.gen::<f64>());

    // 生成随机i32数字
    println!("random i32: {}", rng.gen::<i32>());

    println!("random u8: {}", rng.gen::<u8>());

    println!("random u32: {}", rng.gen::<u32>());

    // 生成区间的随机数[1,10)
    let i: i64 = rng.gen_range(1..10);
    println!("i:{}", i);

    let m: u32 = rng.gen_range(1..100);
    println!("m:{}", m);

    println!("rnd string:{}", rand_string(10));
    println!("diy rnd string:{}", diy_rand_string(10));
    println!("diy rnd string:{}", diy_rand_string(12));
}

// 生成随机字符串(包含0-9,a-z,A-Z所组成)
fn rand_string(size: usize) -> String {
    let rnd_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();
    rnd_string
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

fn diy_rand_string(size: usize) -> String {
    // 通过对CHARSET随机产生一个char，一共生成size个char组成字符串string
    let mut rng = rand::thread_rng();
    let pwd: String = (0..size)
        .map(|_| {
            // 使用用户自定义的字节字符串，使用 gen_range 函数
            let pos = rng.gen_range(0..CHARSET.len());
            CHARSET[pos] as char
        })
        .collect();

    pwd
}
