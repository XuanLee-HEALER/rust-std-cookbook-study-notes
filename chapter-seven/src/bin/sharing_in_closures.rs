use std::{sync::Arc, thread};

/// Arc 和 Rc的区别是，引用计数的方式使用了原子原语（atomic primitives）的方式实现
/// * Arc 比 Rc更慢，不只是简单的加减过程
/// * Arc 可以在线程之间安全使用
///
/// Rust的闭包只能操作三种类型的变量
/// * 传入的参数
/// * static 变量，有 'static 生命周期参数的变量
/// * 它拥有的变量，通过创建或者移动进入闭包
fn main() {
    let some_resource = Arc::new("Hello World".to_string());
    let thread_a = {
        let some_resource = Arc::clone(&some_resource);
        thread::spawn(move || println!("Thread A says: {}", some_resource))
    };
    let thread_b = {
        let some_resource = Arc::clone(&some_resource);
        thread::spawn(move || println!("Thread B says: {}", some_resource))
    };

    // 当你需要它们的结果和不再等待它们时 join 它们，或者它们将要被清理
    thread_a.join().expect("Thread A panicked");
    thread_b.join().expect("Thread B panicked")
}
