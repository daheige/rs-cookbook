use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

use anyhow::Result; // 返回Result<T,E>
use std::{env, fs};
fn main() -> Result<()> {
    // 写入文件使用了Write trait
    let path = "test.md";
    let mut output = File::create(path)?;
    write!(output, "hello\nrust\n")?;
    write!(output, "daheige\ntest")?;

    // 读取文件
    let input = File::open(path)?;
    let buf = BufReader::new(input);
    for line in buf.lines() {
        println!("{}", line?);
    }

    Ok(())
}

#[test]
fn latest_updated_file() -> Result<()> {
    let current_dir = env::current_dir()?;
    println!("current dir:{:?}", current_dir);

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();
        if last_modified < 24 * 3600 && metadata.is_file() {
            if path.file_name().is_none() {
                println!("filename is empty");
                continue;
            }

            println!(
                "last modified:{:?} seconds,is read only:{:?} size:{:?} bytes,filename:{:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().unwrap()
            )
        }
    }

    Ok(())
}
