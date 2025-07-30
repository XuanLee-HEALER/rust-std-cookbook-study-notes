fn main() {
    by_moving();
    by_cloning();
    by_mutating();
}

/// # 移动方式
/// 只要适合，就应该使用这种方式
/// ## 优点
/// * 直观，使用了 `+`
/// * 只能用于不可变数据，尽可能写更少的有状态行为
/// * 重用了 `hello` 分配的内存，性能不错
/// ## 缺点
/// * 原来的 `hello` 变量不能再使用
/// * 有时你真的需要可变数据来表示状态
fn by_moving() {
    let hello = "hello ".to_string();
    let world = "world!";

    // 使用move的方式，将分配的内存和额外的字符串切片移动到一个新变量
    let hello_world = hello + world;
    // hello不能再使用
    println!("{}", hello_world)
}

fn by_cloning() {
    let hello = "hello ".to_string();
    let world = "world!";

    // hello.clone() 分配新内存到临时变量，原来的 `hello` 还能继续用
    // 运行时额外的内存代价
    let hello_world = hello.clone() + world;
    println!("{}", hello_world);
}

fn by_mutating() {
    let mut hello = "hello ".to_string();
    let world = "world!";

    // 性能上和 by_moving 一样
    // 如果不需要维护状态，还是使用 by_moving
    hello.push_str(world);
    println!("{}", hello)
}
