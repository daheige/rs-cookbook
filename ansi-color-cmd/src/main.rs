use ansi_term::{Colour, Style};

// 终端文字添加颜色或样式
fn main() {
    println!(
        "this is {} in color,{} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green"),
    );

    println!("hello,{}", Colour::Green.paint("daheige"));

    println!(
        "{} and this is not",
        Style::new().bold().paint("this is bold")
    );

    // 彩色文本和粗体
    println!(
        "{},{}",
        Colour::Yellow.bold().paint("this is colored"),
        Style::new().bold().paint("this is bold"),
    )
}
