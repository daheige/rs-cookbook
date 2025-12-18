use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::punctuated::Punctuated;
use syn::{Expr, Token, parse_macro_input};

// 派生宏（derive）实现
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap(); // 解析输入
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident; // Name of the struct or enum.
    let g = quote! {
        // 为name实现HelloMacro trait
        // 这里的#name是指定的数据类型
        impl HelloMacroTrait for #name {
            fn hello(){
                println!("Hello, Macro!My name is {}", stringify!(#name));
            }
        }
    };

    g.into()
}

// 属性宏
#[proc_macro_attribute]
pub fn log_calls(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let name = &sig.ident; // 函数名字

    let expanded = quote! {
        #vis #sig {
            let start = std::time::Instant::now();
            println!("entering {}", stringify!(#name));
            // 调用函数
            let _result = (||#block)();
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("exiting {}", stringify!(#name));
            let end = start.elapsed();
            println!("cost time: {:?}", end.as_millis());
            _result
        }
    };

    expanded.into()
}

// 函数过程宏
#[proc_macro]
pub fn make_vec(input: TokenStream) -> TokenStream {
    // 解析输入为逗号分隔的表达式列表
    let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);

    // 将Punctuated转换为Vec，使其可迭代
    let args_vec: Vec<_> = args.into_iter().collect();

    // 生成代码
    let g = quote! {
        {
            let mut v = Vec::new();
            // 使用迭代器语法
            #( v.push(#args_vec); )*
            v
        }
    };
    g.into()
}

/*
小结：
Rust 的“宏”可以从不同角度分类，常见的类别与示例包括：
1.声明式宏（macro_rules!）
2.内置宏（标准库/编译器提供的宏，如 println!, vec!, include_str! 等）
3.过程宏（procedural macros），又分三种子类：
    派生宏（derive）
    属性宏（attribute）
    类函数宏 / 函数样宏（function-like）

1.过程宏必须放在单独的 proc-macro crate（Cargo.toml 中 [lib] proc-macro = true），
不能在同一个普通可执行 crate 里直接定义并使用。
2.确认在 lib.rs 中正确 import 了 proc_macro::TokenStream、syn、quote 等。
3.宏展开的结果必须合法 Rust 代码。
// 补充说明
4.macro_rules! 适合大多数常见模式匹配宏，语法简单、无需额外 crate。
5.过程宏更强大、能做复杂的源码生成或变换，但需要单独 crate，并依赖 syn/quote，调试相对复杂。
6.标准库提供了很多常用宏（println!, format!, vec!, dbg!, include_str!, concat!, stringify!, env! 等），这些通常直接使用。
 */
