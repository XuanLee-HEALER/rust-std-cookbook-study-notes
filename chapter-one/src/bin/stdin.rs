use std::io::{self, BufRead, Write};

fn main() {
    print_single_line("Please enter your forename: ");
    let forename = read_line_iter();

    print_single_line("Please enter your surname: ");
    let surname = read_line_buffer();

    print_single_line("Please enter your age: ");
    let age = read_number();

    println!(
        "Hello, {} year old human named {} {}!",
        age, forename, surname
    )
}

fn print_single_line(text: &str) {
    print!("{}", text);
    // 为了保证数据被显示需要显式flush标准输出
    io::stdout().flush().expect("Failed to flush stdout")
}

fn read_line_iter() -> String {
    // 返回的对象是一个对全局 stdin 对象的引用，这个全局buffer通过一个Mutex管理
    let stdin = io::stdin();
    // 以迭代器风格从标准输入读取一行
    // 我们通过锁住buffer来访问这个handle
    let input = stdin.lock().lines().next();
    input
        .expect("No lines in buffer")
        .expect("Failed to read line")
        .trim()
        .to_string()
}

fn read_line_buffer() -> String {
    let mut input = String::new();
    // read_line方式不需要调用lock函数，因为它会隐式调用
    // 最好使用上面那种迭代器的方式，应为要避免使用状态（可变）
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_number() -> i32 {
    let stdin = io::stdin();
    loop {
        for line in stdin.lock().lines() {
            let input = line.expect("Failed to read line");
            match input.trim().parse::<i32>() {
                Ok(num) => return num,
                Err(e) => println!("Failed to read number: {}", e),
            }
        }
    }
}
