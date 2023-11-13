#[warn(dead_code)]
use anyhow::{bail, Context, Result};
use std::fmt::Display;
use std::panic::Location as sLocation; // 这个一般是用于panic hook使用
use std::{env, fs, io};

// trace
use tracing::{error, info, Level};
use tracing_subscriber;

// 自定义宏，传递给anyhow返回的Result<T, E>对应的context方法
macro_rules! file_line_here {
    () => {
        concat!("at ", file!(), " line ", line!(), " column ", column!())
    };
}

// 定义错误信息
pub struct Location {
    file: &'static str,
    line: u32,
    column: u32,
}

pub trait ErrorLocation<T, E> {
    fn location(self, loc: &'static Location) -> Result<T>;
}

impl<T, E> ErrorLocation<T, E> for Result<T, E>
where
    E: Display,
    Result<T, E>: Context<T, E>,
{
    fn location(self, loc: &'static Location) -> Result<T> {
        let msg = self.as_ref().err().map(ToString::to_string);
        self.with_context(|| {
            format!(
                "{} at {} line {} column {}",
                msg.unwrap(),
                loc.file,
                loc.line,
                loc.column,
            )
        })
    }
}

macro_rules! here {
    () => {
        &Location {
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

fn foo() -> Result<()> {
    bail!("oh,no");
}

fn f() -> Result<()> {
    let s = fs::read_to_string("test1.md")?;
    println!("s:{}", s);
    Ok(())
}

fn main() -> Result<()> {
    println!("Hello, world!");
    // foo().context(file_line_here!())?;
    // f().context(file_line_here!())?;

    // 通过std::panic::Location获取file和line
    // 获取当前文件和行数
    println!(
        "file:{} line:{}",
        sLocation::caller().file(),
        sLocation::caller().line()
    );
    println!(
        "file:{} line:{}",
        sLocation::caller().file(),
        sLocation::caller().line()
    );

    // 使用自定义的error location方法，这种模式，无法进一步解开底层的error file和line
    // f().location(here!())?;

    // 初始化trace配置
    env::set_var("RUST_LOG", "debug"); // 设置日志级别
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_file(true)
        .with_line_number(true)
        .with_writer(io::stdout) // 写入到标准输出
        // sets this to be the default, global collector for this application.
        .init();
    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");
    // let _ = f().map_err(|err| {
    //     error!("err:{}", err);
    // });

    // 这里是f()调用的位置发生错误，因为f运行错误返回了err
    let _ = f().map_err(|err| error!("err:{}", err));

    Ok(())
}
