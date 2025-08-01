use glob::{MatchOptions, glob, glob_with};

/// glob(...) 你可以为所有匹配的文件构造一个迭代器，可以把 glob 模式看作简化的正则表达式，主要用于文件名。语法介绍参考 wikipedia
/// glob迭代器返回的也是 Result 因为程序可能没有权限访问文件系统
///
/// 使用 glob_with 可以指定一个 MatchOptions 实例来改变 glob 的搜索方式，最有用的选项包括
/// * `case_sensitive` 大小写字母是否应该被一致对待，默认开启
/// * `require_literal_leading_dot` 默认关闭，开启后通配符不再匹配文件名前面的 . 当你想忽略隐藏文件时很有用
fn main() {
    println!("All all Rust files in all subdirectories:");
    for entry in glob("**/*.rs").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("Failed to read file: {:?}", e),
        }
    }

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_leading_dot: true,
        ..Default::default()
    };

    println!(
        "All files that contain the word \"ferris\" case insensitive \
    and don't contain an underscore:"
    );
    for entry in glob_with("*Ferris[!_]*", options).expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            println!("{:?}", path.display())
        }
    }
}
