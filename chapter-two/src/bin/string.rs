/// 因为字符串也是一种vector，所以可以使用 new 和 push 创建，但是很不方便
/// string有自己的内存，可以从字符串切片创建，应该使用 to_string（性能和to_owned相同）
/// Unicode可以让你从不同的Unicode Scalar Value组成字符（character）。chars()会创建一个迭代器，来遍历这些标量 unicode-segmentation crate可以在“实际字符”上遍历
///
/// 划分函数中的模式（pattern），包含
/// * 一个字符
/// * 一个string
/// char提供的一些predicate
fn main() {
    // String是一种vector
    let mut s = String::new();
    s.push('H');
    s.push('i');
    println!("s: {}", s);

    // 以下两种方式是等效的
    let s = "Hello".to_string();
    println!("s: {}", s);
    let s = String::from("Hello");
    println!("s: {}", s);

    // Rust中的String总是有效的UTF-8
    let s = "汉语 😊🍺".to_string();
    println!("s: {}", s);

    let mut s = "Hello ".to_string();
    s.push_str("World");

    // 在字符上遍历，这里的字符是 Unicode Scalar Value
    for ch in "Tubular".chars() {
        print!("{}.", ch);
    }
    println!();

    for ch in "y?".chars() {
        print!("{} ", ch);
    }
    println!();

    let (first, second) = "HelloThere".split_at(5);
    println!("first: {}, second: {}", first, second);

    let haiku = "\
she watches\n\
satisfied after love\n\
he lies\n\
looking up at nothing\n\
    ";
    for line in haiku.lines() {
        println!("\t{}.", line);
    }

    for s in "Never;Give;Up".split(';') {
        println!("{}", s);
    }

    // 当划分字符串在开头或者结尾，会产生空字符串
    let s: Vec<_> = "::Hi::There::".split("::").collect();
    println!("{:?}", s);

    // 消除末尾的空字符串：如果最后一个字符串是分隔符，则消除之后生成的空字符
    let s: Vec<_> = "Mr. T.".split_terminator('.').collect();
    println!("{:?}", s);

    // char有一些方法，可以用来划分字符串
    for s in "I'm2fast4you".split(char::is_numeric) {
        println!("{}", s);
    }

    // 只划分有限次
    for s in "It's not your fault, it's mine".splitn(3, char::is_whitespace) {
        println!("{}", s);
    }

    // 只获取匹配模式的子串，是划分串的反面
    for c in "The Dark Knight rises".matches(char::is_uppercase) {
        println!("{}", c);
    }

    let saying = "The early bird gets the worm";
    let starts_with_the = saying.starts_with("The");
    println!(
        "Does \"{}\" start with \"The\"?: {}",
        saying, starts_with_the
    );
    let starts_with_bird = saying.starts_with("bird");
    println!(
        "Does \"{}\" start with \"bird\"?: {}",
        saying, starts_with_bird
    );

    let ends_with_worm = saying.ends_with("worm");
    println!("Does \"{}\" end with \"worm\"?: {}", saying, ends_with_worm);

    let contains_bird = saying.contains("bird");
    println!("Does \"{}\" contain \"bird\"?: {}", saying, contains_bird);

    let a_lot_of_whitespace = "   I   love spaaace       ";
    let s: Vec<_> = a_lot_of_whitespace.split(' ').collect();
    println!("{:?}", s);
    let s: Vec<_> = a_lot_of_whitespace.split_whitespace().collect();
    println!("{:?}", s);

    let username = "    P3ngu1n\n".trim();
    println!("{}", username);
    let username = "    P3ngu1n\n".trim_start();
    println!("{}", username);
    let username = "    P3ngu1n\n".trim_end();
    println!("{}", username);

    let num = "12".parse::<i32>();
    if let Ok(num) = num {
        println!("{} * {} = {}", num, num, num * num);
    }

    let s = "My dad is the best dad";
    let new_s = s.replace("dad", "mom");
    println!("new_s: {}", new_s);

    let lowercase = s.to_lowercase();
    println!("lowercase: {}", lowercase);

    let uppercase = s.to_uppercase();
    println!("uppercase: {}", uppercase);

    let chinese = "你好";
    println!("lowercase chinese: {}", chinese.to_lowercase());

    let hello = "Hello! ";
    println!("Three times hello: {}", hello.repeat(3));
}
