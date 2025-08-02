use chapter_five_derive::HelloWorld;

/// 提供一个自定义的 derive 需要创建一个过程宏。过程宏（procedural macro）是编译器运行的代码，并且直接和编译器交互，它需要在单独的crate中
/// 并且设置
///
/// ```toml
/// [lib]
/// proc-macro = true
/// ```
///
/// 新版本的 syn 和 quote 和书里的用法差别比较大，不能直接测试
///
/// quote crate可以让Rust代码转换成编译器可以使用的token，这个宏很有用的特性是通过在前面加上 # 可以支持变量的代码插值，即某个变量的值会变成Rust代码
/// syn crate是一个Rust的转换器（parser），基于 nom 转换器组合框架。syn将自定义的属性或者derive注解的代码转换成AST
fn main() {
    Switzerland::hello_world();
    Britain::hello_world();
    Australia::hello_world();
}

trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
struct Switzerland;

#[derive(HelloWorld)]
struct Britain;

#[derive(HelloWorld)]
#[hello_world_name = "the Land Down Under"]
struct Australia;
