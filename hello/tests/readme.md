# tests 集成测试
单元测试独立地验证库的不同部分，也能够测试私有函数实现细节。
集成测试则检查多个部分是否能结合起来正确地工作，并像其他外部代码那样测试库的公有 API。
即使 Rust 的类型系统和所有权规则可以帮助避免一些 bug，
不过测试对于减少代码中不符合期望行为的逻辑 bug 仍然是很重要的。

在项目的根目录下新建tests目录，然后把对应的单元测试写入一个文件即可，比如：add_test.rs

# 测试
```shell
cargo test --test  add_test -- --show-output
```
输出结果：
```
running 1 test
test add ... ok

successes:

---- add stdout ----
hello,tests
13


successes:
    add

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

如果不需要输出内容，就使用下面的方式
```shell
cargo test --test add_test
```

测试集成测试某个文件中的某个单元函数
```shell
cargo test --test  add_test add
```
输出结果：
```
 Finished test [unoptimized + debuginfo] target(s) in 0.08s
     Running tests/add_test.rs (~/web/rust/rs-cookbook/target/debug/deps/add_test-9c8144cde11efb4d)

running 1 test
test add ... ok
```
