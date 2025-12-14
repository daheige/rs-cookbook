# 在c代码中调用rust定义的ffi函数
通过crate-type 方式实现

# rust类型和c类型对应关系
Rust类型与C类型映射
Rust类型	C类型
i32	    int
u32	    unsigned int
i64	    long long
u64	    unsigned long long

## 默认整数类型‌
- Rust中i32是默认的有符号整数类型（32位），对应C语言的int（通常是32位）。
- C语言的int大小取决于编译器和平台（32位或64位），但i32始终为32位。
## 类型安全
- Rust通过类型系统确保类型安全，i32只能存储32位整数，超出范围会溢出（默认行为）。
- C语言的int大小不固定，需使用int32_t（需包含stdint.h）确保32位。

小结：
i32在C语言中直接对应int，但需注意C语言的int大小不固定，建议使用int32_t确保32位兼容性。

# 在c语言中调用rust代码

1. 通过cargo new --lib mylib生成一个rust library库
2. 在Cargo.toml中添加如下内容配置

```toml
[lib]
name = "rslib" # 链接库名字
#配置 Rust 生成两种库——静态库（staticlib）和 C动态库（cdylib）
crate-type = ["staticlib", "cdylib"]
```

3. 在lib.rs添加如下代码

```rust
// 对外提供add函数供c语言调用
#[unsafe(no_mangle)]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_rust() {
    println!("called call_from_rust!");
}

// 关闭混淆功能以让c程序可以找到调用的函数
// extern 默认导出为C ABI
#[unsafe(no_mangle)]
pub extern "C" fn say_hello(){
    println!("call func from rust");
    println!("hello,{}","world");
}
```

4. 在src目录下新建一个rslib.h头文件，添加如下代码：

```c
// 为了给 C 调用我们还需要编写一个头文件
// 在这个头文件中定义函数签名
void say_hello();
void call_from_rust();
int add(int left,int right);
```

5. 在src目录下新建一个main.c文件，添加如下代码：

```c
#include "rslib.h"
#include <stdio.h>

int main() {
    // 调用rust mylib库中的say_hello函数
    // 在编译的时候可以选择是用动态链接或静态链接的方式来生成c语言的二进制文件
    // 具体生成机制看bin/share-run.sh和static-run.sh即可
    say_hello();
    call_from_rust();
    int c = add(1,2);
    printf("add(1,2)=%d\n",c);
}
```

6. 开始编译，这里我使用采用shell脚本的方式实现cargo build和gcc编译，分别执行如下shell脚本运行程序
- 构建rust library
```shell
sh bin/build.sh
```

- 调用rust提供的ffi外部函数
```shell
# 动态链接的方式编译构建c二进制文件main，并运行
sh bin/share-run.sh

# 静态链接的方式编辑构建c二进制文件main-hello,并运行
sh bin/static-run.sh
```
运行结果如下：
```ini
call func from rust
hello,world
called call_from_rust!
add(1,2)=3
```

# cc crate
- https://crates.io/crates/cc
- https://github.com/rust-lang/cc-rs

该cc库可以支持更多复杂的c/c++函数在rust调用。