use std::{collections::HashMap, sync::RwLock};

use lazy_static::lazy_static;
use regex::Regex;

/// lazy_static! 宏可以扩展Rust的 static 功能，后者需要你创建在编译期可以构造的对象，前者可以创建在运行时初始化的懒对象
/// 通过调用 lazy_static! ，我们在**当前域**定义了一个懒初始化对象，只有在第一次使用时才会被创建
///
/// > 如果你要声明的是内容已知的vector，可以使用常量数组，因为它的构建是编译期的
///
/// 新版本（1.7）之后可以使用OnceLock/LazyLock结构替代此crate的功能
///
/// Rust中，static变量是在程序运行期间存活的变量，所以它们有自己的生命周期 `'static` ，所以它们的必须以constant way创建（编译期）
/// 为什么 CURRENCIES 不能用普通的 static 声明？因为HashMap::new()返回的新对象是在运行时内存的某个位置，首先程序必须已经在内存中，所以这在编译期不可能发生
/// 另一个问题是，对于 'static 生命周期的对象，借用检查器不能确定它是线程安全的，所以任何对 static mut 变量的访问都是 unsafe 的
///
/// lazy_static 将你的对象包裹进新创建的结构体中，可以被隐式解引用为你的对象。也就是你永远不需要直接访问你的对象。因此它使用 ref 来强调这个过程，你会将这个变量
/// 看作一个引用，解引用时，这个包裹体结构体会在你的动态创建的对象中操作 static mut 指针。它只是将 unsafe 调用以safe的方式暴露出来
fn main() {
    let usd = CURRENCIES.get("USD");
    if let Some(usd) = usd {
        println!("USD stands for {}", usd)
    }

    if let Some(chf) = CURRENCIES.get("CHF") {
        println!("CHF stands for {}", chf)
    }

    CLIENTS
        .write()
        .expect("Failed to unlock clients for writing")
        .push("192.160.0.1".to_string());

    let clients = CLIENTS
        .read()
        .expect("Failed to unlock clients for reading");
    let first_client = clients.get(0).expect("CLIENT is empty");
    println!("The first client is: {}", first_client);

    let date = "12.01.2018";
    if let Some(day) = extract_day(date) {
        println!("The date \"{}\" contains the day \"{}\"", date, day)
    }
}

// 全局不可变 static
lazy_static! {
    static ref CURRENCIES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("EUR", "Euro");
        m.insert("USD", "U.S. Dollar");
        m.insert("CHF", "Swiss Francs");
        m
    };
}

// 全局可变 static
lazy_static! {
    static ref CLIENTS: RwLock<Vec<String>> = RwLock::new(Vec::new());
}

// 局部 static
// 这种用法很常见，对于初始化过程代价高昂的操作，可以使用这种方式只初始化一次
fn extract_day(date: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\d{2}).(\d{2}).(\d{4})").expect("Failed to create regex");
    }
    RE.captures(date)
        .and_then(|cap| cap.get(1).map(|day| day.as_str()))
}
