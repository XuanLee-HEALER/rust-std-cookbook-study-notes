use std::env;

/// 根据Twelve-Factor App，https://12factor.net/，你应该在环境变量中存储你的配置
/// 这意味着你在不同的部署环境中可以改变传递的值，例如端口号、域、数据库信息
/// 很多程序也使用环境变量和其它程序通信
///
/// 更好的方式是创建一个文件 .env 包含 k-v对形式的配置，在构建的时候加载到进程中
/// dotenv crate
fn main() {
    // 遍历当前进程的所有环境变量
    println!("Listing all env vars: ");
    // env::vars()，访问执行时为当前进程设置的所有环境变了
    for (key, val) in env::vars() {
        println!("{}: {}", key, val);
    }

    let key = "PORT";
    println!("Setting env var {}", key);
    // 为当前进程设置环境变量
    unsafe {
        // 只会影响当前进程的环境变量
        env::set_var(key, "8080");
    }

    print_env_var(key);
}

fn print_env_var(key: &str) {
    // 返回单个环境变量，如果不存在或者值包含无效的Unicode会报错
    // .unwrap_or_default("") 可以很方便地提供一个默认值
    match env::var(key) {
        Ok(val) => println!("{}: {}", key, val),
        Err(e) => println!("Couldn't print env var {}: {}", key, e),
    }
}
