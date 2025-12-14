# c语言调用rust定义的函数
通过ffi外部函数方式实现，这里我使用了`cbindgen`自动生成了c语言的头文件，例如:rust_interop.h

# 运行方式
1. 构建rust library库
```shell
cargo build
```
此时在当前项目中会自动`rust_interop.h`头文件，具体实现见：`build.rs`
2. 执行如下命令运行c二进制程序
```shell
sh bin/share-run.sh
```
当rust代码代码发生变更时候，请记得先执行第一步，再运行该脚本。

# rust和c字符串映射关系
关键说明：
## ‌类型映射‌：
- Rust &str → C const char*
- Rust String → C char*
- Rust CString → C char*（需手动释放）
## ‌内存管理‌：
- Rust生成的C字符串需手动释放（free_c_string）
- C生成的Rust字符串需手动释放（CString::from_raw）

## ‌错误处理‌：
- CString::new会检查'\0'字符，避免C字符串截断
- 使用CStr::from_ptr确保C字符串安全转换

# 小结
- 通过 cbindgen crate可以非常方便的将rust定义的函数作为c语言外部函数调用，同时解决了头文件的编写问题。
- 对于rust对外提供的C ABI函数，增加 `#[unsafe(no_mangle)]`注解来告诉 Rust 编译器不要 mangle 此函数的名称，避免在调用过程中，找不到函数名字。
- 使用 `extern "C"` 来创建一个允许其它语言调用 Rust 函数的接口，这里使用的是C ABI二进制接口方式，保证了稳定性和可靠性。
- 如果需要在rust中调用复杂的c/c++函数，建议使用`cc` crate这个组件。