# log4rs
- log4rs 将日志输出配置到自定义位置。log4rs 可以使用外部 YAML 文件或生成器配置。

- 使用文件附加器log4rs::append::file::FileAppender 创建日志配置，文件附加器
定义日志记录的目标位置。日志配置使用 log4rs::encode::pattern 中的自定义模式进行编码，
将配置项分配给 log4rs::config::Config，并设置默认的日志等级 log::LevelFilter。

https://docs.rs/log4rs/latest/log4rs/
