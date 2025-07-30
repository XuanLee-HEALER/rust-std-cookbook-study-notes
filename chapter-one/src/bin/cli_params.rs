use std::env;

fn main() {
    println!("Got following parameters: ");
    for arg in env::args() {
        println!("- {}", arg)
    }

    // 使用迭代器访问某个参数
    // 使用迭代器强迫你在编译期进行有效性检查
    let mut args = env::args();
    // 大部分OS的第一个命令行参数都是执行程序本身
    if let Some(arg) = args.nth(0) {
        println!("The path to this program is: {}", arg)
    }
    if let Some(arg) = args.nth(1) {
        println!("The first parameter is: {}", arg)
    }
    // 之前的值已经被消费，这里不会打印
    if let Some(arg) = args.nth(2) {
        println!("The second parameter is: {}", arg)
    }

    // 使用集合
    let args: Vec<_> = env::args().collect();
    println!("The path to this program is: {}", args[0]);
    if args.len() > 1 {
        println!("The first parameter is: {}", args[1])
    }
    if args.len() > 2 {
        println!("The second parameter is: {}", args[2])
    }
}
