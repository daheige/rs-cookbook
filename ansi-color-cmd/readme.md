# ansi_term 终端文本颜色设置
- ansi_term 中有两种主要的数据结构：ANSIString 和 Style。Style 包含样式信息：颜色，是否粗体文本，或者是否闪烁，或者其它样式。
- 还有 Colour 变量，代表简单的前景色样式。ANSIString 是与 Style 配对的字符串。
- 打印彩色文本到终端实用
```rust
println!(
        "this is {} in color,{} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green"),
    );
```
- 对于比简单的前景色变化更复杂的事情，代码需要构造 Style 结构体
```rust
 println!(
        "{} and this is not",
        Style::new().bold().paint("this is bold")
    );
```

运行效果如下：
![img.png](img.png)
