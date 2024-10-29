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

# rust commonly used crates
- tokio 异步运行时，除了复杂上手难度高一些外，没有其它大的问题
- serde 一个超高性能的通用序列化/反序列化框架，可以跟多种协议的库联合使用，实现统一编解码格式
- serde_json 用于json序列化和反序列化
- serde_yaml用于yaml文件读写操作，dtolnay/serde-yaml 使用serde编解码YAML格式的数据
- sqlx 用于数据库db操作
- redis-rs 库用于redis操作
- r2d2连接池管理，用于mysql redis连接池管理
- kafka-rust/kafka-rust 相比上一个库，它算是纯Rust实现，文档还行，支持Kafka0.8.2及以后的版本，但是对于部分0.9版本的特性还不支持
- pulsar 用于pulsar消息队列操作
- kafka 用于rust kafka client操作
- anyhow用于error handler处理result
- hyperium/tonic 纯Rust实现的gRPC客户端和服务器端，支持async/await异步调用，用于grpc微服务开发
- tokio-rs/prost 用来代码生成，tokio出品
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
- tafia/quick-xml 高性能XML库，可以配合serde使用，文档较为详细
- napi 用于rust为nodejs写拓展的库
- neon 用于rust 为 node.js 写拓展的库
- pyo3 用于rust 为 python 写拓展的库
- rustwasm 用于rust 开发wasm的库
- wasm-bindgen 用于wasm开发
- cloudflare/quiche 大名鼎鼎cloudflare提供的QUIC实现，据说在公司内部重度使用，有了大规模生产级别的验证，非常值得信任，同时该库还实现了HTTP/3
- md5 用于rust md5加密算法
- rand 用于生成随机数或字符串的rand库
- mime 用于文件类型的库
- rayon 用于并行计算的库
- mongodb 用于mongodb操作的库
- chrono 用于时间操作的库
- base64 用于Rust中字符串的base64编码与解码
- hmac-sha256 用于rust hmac 256处理
- rust-crypto 用于aes,bcrypt,blowfish,ecb,cbc,hmac,md5,sha1等加密/解密处理的库
- num_cpus 获取当前cpu个数
- regex 用于正则表达操作的库
- semver 用于版本比较的库
- sha256 用于sha256算法的库
- structopt 用于结构体配置读取的库
- thiserror 用于包装error的库
- url 用于url处理的库
- uuid 用于生成uuid的库
- validator 用于参数校验的库
- lru 用于lru cache本地缓存的库
- dotenv 用于读取.env文件的库
- html-escape 处理html标签转义
- jsonwebtoken 用于web jwt认证的库

# 更多的crates
https://crates.io 可以在这个网站上面进行搜索即可

# rust交叉编译工具cross
cross compile，通过设置编译时所使用的目标（target），可以实现让Rust在同样的系统环境下编译出不同系统环境的程序。
然而，要实现系统原生程序的交叉编译不是一件简单的事，因为会受到专利、技术、平台依赖等限制，实际中并不是所有的程序都可以被成功的交叉编译。
Rust提供多种编译目标，纯粹的Rust代码可以很容易的实现交叉编译，但是如果Rust程序中有用到C/C++函数库，情况就会变得非常复杂。
我这这里只讨论纯Rust代码的交叉编译。

## cross 安装和使用
cross是一个Rust交叉编译的项目，其项目地址: https://github.com/cross-rs/cross
它利用Docker简化了在x86_64的Linux操作系统上进行交叉编译时所需要的前置设置。
提供了多种常见的CPU架构和部分操作系统的交叉编译环境，除了Rust代码能够交叉编译外，
因为包含C/C++编译器，所以C/C++的代码也可以跟着一起进行交叉编译。
在使用cross工具之前，需要事`先安装Docker与Rust开发环境`，输入以下指令，可直接使用Cargo来安装cross工具：
```shell
cargo install cross

# 测试是否配置成功
cross --version
cross 0.2.5
```

安装完成后，如果需要进行交叉编译，就可以使用cross build 、cross check、cross run、cross test
来代替cargo build、cargo check、cargo run、cargo test，在通过传入--target参数来指定要交叉编译成哪个系统环境的程序。
使用方法：
```shell
# 在构建平台上交叉编译出Windows x86_64程序
cross build --target x86_64-pc-windows-gnu

# 在构建平台上交叉编译出64位的Linux ARM程序
cross build --target aarch64-unknown-linux-gnu

# Linux X86_64架构，无外部依赖，支持centos,ubuntu系统
# 在部分mac机器上，编译不成功，可能是缺少 musl 部分组件导致的
# 当遇到这种情况，可以选择musl工具链: cargo build --target=x86_64-unknown-linux-musl 发布编译
cross build --target x86_64-unknown-linux-musl

# 在构建平台上交叉编译Linux x86_64程序，gnu需要glibc
cross build --target x86_64-unknown-linux-gnu

```

# cross运行
cross 运行参考手册：https://github.com/cross-rs/cross/wiki/Getting-Started

- 运行在 `aarch64-unknown-linux-gnu` 上
```shell
cross run --target aarch64-unknown-linux-gnu
info: downloading component 'rust-src'
info: installing component 'rust-src'
    Finished dev [unoptimized + debuginfo] target(s) in 2.49s
     Running `/linux-runner aarch64 /target/aarch64-unknown-linux-gnu/debug/rand-demo`
Hello, world!
```

- 运行在 `x86_64-unknown-linux-gnu` 上，要求运行环境有glibc
对于`x86_64-unknown-linux-musl` 是无任何外部依赖的linux x86_64架构
```shell
% cross run --target x86_64-unknown-linux-gnu
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `/linux-runner x86_64 /target/x86_64-unknown-linux-gnu/debug/rand-demo`
Hello, world!
```

- 对于 `x86_64-unknown-linux-musl` 目前cross 编译运行，在有些mac机器上不可执行，可以通过 cargo musl工具链的方式交叉编译，见下面的 musl 交叉编译。
```shell
% cargo build --target=x86_64-unknown-linux-musl
```

# 在MacOS上使用musl实现Linux交叉编译

- 绝大部分的Rust程序员都会有跟我一样的需求，写代码用的是Windows或者Mac部署平台是Linux，这种情况下就需要使用cross交叉编译意思是可以在当前平台Host下编译出目标平台target的可执行文件，尤其是做ARM平台开发的同学对这个更为熟悉。

- Rust交叉编译在Github上有一个文档Rust核心员工Jorge Aparicio提供的一份文档 https://github.com/japaric/rust-cross 推荐大家仔细的读一读。

- 对我而言，我的要求比较简单，都是X86_64架构，从Mac上编译出unknow-linux就好(因为大部分服务器主要是centos/ubuntu操作系统为主)

musl实现了Linux libc，质量可靠，适配所有Linux环境，使用静态链接替换动态链接，这样就能交叉编译一个完整的二进制文件，可以丢到任何Linux环境里运行。当然，关于静态链接与动态链接各有优缺点，这里不细说。
下面就以MacOS操作系统为例，实现Linux交叉编译：
1. 首先安装一下 `x86_64-unknown-linux-musl`
    ```shell
        rustup target add x86_64-unknown-linux-musl
    ```
    vim  main.rs

    ``` rust
        fn main() {
            println!("Hello, world!");
        }
    ```
2. 添加配置 vim ~/.cargo/config添加如下内容：
    ```toml
    [target.x86_64-unknown-linux-musl]
    linker = "rust-lld"
    rustflags = ["-C", "linker-flavor=ld.lld"]
    ```

    ```shell
    cargo build --release --target=x86_64-unknown-linux-musl
    ```
    成功就会输出：Finished release [optimized] target(s) in 0.00s

3. 编译好的结果会放入 `target/x86_64-unknown-linux-musl/release`中
    把结果丢到Linux下执行，没问题
    ```
    $ ./hello
    Hello, world!
    ```

# 在MacOS上使用musl交叉编译的常见问题

要是提示
```shell
/bin/sh: musl-gcc: command not found
```
解决方法是安装musl-cross
```shell
brew install filosottile/musl-cross/musl-cross
```
配置config
$ vim .cargo/config
``` toml
[target.x86_64-unknown-linux-musl]
linker = "rust-lld"
rustflags = ["-C", "linker-flavor=ld.lld"]
```

要是你的程序依赖原生库，需要设置一个环境变量 CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc
所以完整的编译命令如下:
```shell
CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl
```

要是你的程序使用了OpenSSL类库，这是一个麻烦的事情，目前普遍做法是在Cargo.toml文件中添加依赖:
``` toml
[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
```

# 在Linux或MacOS上实现Windows交叉编译
需要安装工具 cargo-xwin
1. 安装`x86_64-pc-windows-msvc`
```shell
rustup target add x86_64-pc-windows-msvc
```
2. 安装xwin
```shell
cargo install cargo-xwin
```

3. 在~/.cargo/config配置文件中加入如下内容
```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```

4. 使用下面的命令编译即可，它会生成一个xxx.exe文件在target/x86_64-pc-windows-msvc/debug中
```shell
cargo xwin build --target x86_64-pc-windows-msvc
```

# rust交叉编译选择
一般来说推荐使用cross进行交叉编译，避免了各种环境的依赖问题。下面是不同平台交叉编译的选择：
- 对于linux x86_64 架构，建议使用 `x86_64-unknown-linux-musl` 类型进行编译，这样的话，部署就无任何依赖；
```shell
cargo build --target=x86_64-unknown-linux-musl
```

- 如果你的linux系统有glibc库，也可以使用 `x86_64-unknown-linux-gnu` 类型编译，然后部署。
```shell
cross build --target x86_64-unknown-linux-gnu
```

- 对于 window 平台，建议使用 cargo-xwin 或者 cross 编译

    - cargo xwin编译：
    ```shell
    cargo xwin build --target x86_64-pc-windows-msvc
        Finished dev [unoptimized + debuginfo] target(s) in 0.33s
    ```
    - cross编译
    ```shell
    cross build --target x86_64-pc-windows-gnu
    ```
