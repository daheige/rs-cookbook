# rs-cookbook
rust cookbook practical code
- https://www.rustwiki.org.cn/zh-CN/rust-cookbook/intro.html
- https://rust-lang-nursery.github.io/rust-cookbook/

rust version >= 1.58.0
I am currently using version 1.68.1
If you're using version 1.68.0, which has some bugs with log crates, it's advisable to upgrade to a later version, such as v1.68.1
# rust version update
rustup update

# rust-bible
- https://github.com/daheige/rust-bible

# rust常用的crates
- tokio 异步运行时，除了复杂上手难度高一些外，没有其它大的问题
- serde 一个超高性能的通用序列化/反序列化框架，可以跟多种协议的库联合使用，实现统一编解码格式
- serde_json 用于json序列化和反序列化
- serde_yaml用于yaml文件读写操作，dtolnay/serde-yaml 使用serde编解码YAML格式的数据
- sqlx 用于数据库db操作
- redis-rs 库用于redis操作
- r2d2连接池管理，用于mysql redis连接池管理
- kafka-rust/kafka-rust 相比上一个库，它算是纯Rust实现，文档还行，支持Kafka0.8.2及以后的版本，但是对于部分0.9版本的特性还不支持
- anyhow用于error handler处理result
- hyperium/tonic 纯Rust实现的gRPC客户端和服务器端，支持async/await异步调用，用于grpc微服务开发
- tokio-rs/prost 用来代码生成tokio出品
- actix-web用于web api开发，基于Actor模型的异步网络库的router
- axum 用于web api开发，比较好用的路由router库，基于tokio生态的
- tide 基于async-std运行时的http router，用于http web/api开发
- reqwest 用于http请求的高质量库
- env_logger日志库，需要引入log库才可以使用
- clap用于命令行操作的库
- once_cell用于单例模式初始化全局变量，和lazy_static库功能一样
- tokio-rs/tracing 强大的日志框架，同时还支持OpenTelemetry格式，无缝打通未来的监控
- async-std 跟标准库API很像的异步网络库，相对简单易用，但是貌似开发有些停滞，还有就是功能上不够完善。但是对于普通用户来说，这个库非常值得一试，它在功能和简单易用上取得了很好的平衡。
- BurntSushi/rust-csv 高性能CSV读写库，支持Serde
- alexcrichton/toml-rs TOML编码/解码，可以配合serde使用
-  tafia/quick-xml 高性能XML库，可以配合serde使用，文档较为详细
- napi-rs用于rust为nodejs写拓展的库
- neon 用于Rust 为 Node.js 写拓展的库
- rustwasm 用于rust 开发wasm的库
- cloudflare/quiche 大名鼎鼎cloudflare提供的QUIC实现，据说在公司内部重度使用，有了大规模生产级别的验证，非常值得信任，同时该库还实现了HTTP/3
- md5 用于rust md5加密算法
- mime 用于文件类型的库
- rayon 用于并行计算的库
- mongodb 用于mongodb操作的库
- chrono 用于时间操作的库
- base64 用于Rust中字符串的base64编码与解码
- hmac-sha256 用于rust hmac 256处理
- num_cpus 获取当前cpu个数
- regex 用于正则表达操作的库
- semver 用于版本比较的库
- sha256 用于sha256算法的库
- structopt 用于结构体配置读取的库
- thiserror 用于包装error的库
- url 用于url处理的库
- uuid 用于生成uuid的库
- validator 用于参数校验的库

# 更多的crates
https://crates.io 可以在这个网站上面进行搜索即可
