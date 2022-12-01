use std::env;
// use std::fs;
use std::io::Error;

// 读取环境变量
fn main() -> Result<(), Error> {
    let config_path = env::var("CONFIG_PATH").unwrap_or("./config".to_string());
    println!("config_path: {}", config_path);

    // do other things...

    Ok(())
}
