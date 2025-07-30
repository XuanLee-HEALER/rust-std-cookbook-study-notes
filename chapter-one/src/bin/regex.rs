use regex::{Regex, RegexBuilder};

/// 正则表达式的工作方式是在创建时将传入的字符串编译为等效的rust代码
/// 为了更好的性能，你需要尽量重用创建的正则表达式，比较好的办法是使用 laze_static
///
/// ⚠️不要过度使用正则表达式，如果你在转换很复杂的数据，正则表达式也会变得非常复杂
/// 当你的正则表达式已经变得太大而难以理解，应该换成一个parser
fn main() {
    // raw string，字符串中的所有符号都以其字面值表示，而不进行转义。因为\在正则表达式中的含义很多
    // 所有正则表达式都应该使用 r#""# 来写
    let date_regex = Regex::new(r#"^\d{2}.\d{2}.\d{4}$"#).expect("Failed to create regex");
    let date = "15.10.2017";
    let is_date = date_regex.is_match(date);
    println!("Is '{}' a date? {}", date, is_date);

    let date_regex = Regex::new(r#"(\d{2}).(\d{2}).(\d{4})"#).expect("Failed to create regex");
    let text_with_dates = "Alan Turing was born on 23.06.1912 and died on 07.06.1954.\
    A movie about his life called 'The Imitation Game' came out on 14.11.2017";
    // 我们可以遍历匹配到的内容组成的集合，每个集合的 0 位置都是它本身。要注意访问索引没有编译期检查，可能会panic
    for cap in date_regex.captures_iter(text_with_dates) {
        println!("Found date {}", &cap[0]);
        println!("Year: {} Month: {} Day: {}", &cap[3], &cap[2], &cap[1]);
    }
    println!("Original text:\t\t{}", text_with_dates);
    let text_with_indian_dates = date_regex.replace_all(text_with_dates, "$1-$2-$3");
    println!("In indian format:\t{}", text_with_indian_dates);

    // ?P<somename>可以给捕获的组一个名称使得替换组变得更容易
    let date_regex = Regex::new(r#"(?P<day>\d{2}).(?P<month>\d{2}).(?P<year>\d{4})"#)
        .expect("Failed to create regex");
    let text_with_american_dates = date_regex.replace_all(text_with_dates, "$month/$day/$year");
    println!("In american format:\t{}", text_with_american_dates);

    // 使用 (?flag) 来指定匹配选项
    let rust_regex = Regex::new(r#"(?i)rust"#).expect("Failed to create regex");
    println!("Do we match RuSt? {}", rust_regex.is_match("RuSt"));

    // 和上面的正则表达式效果一样，但是更啰嗦，更直观
    let rust_regex = RegexBuilder::new(r#"rust"#)
        .case_insensitive(true)
        .build()
        .expect("Failed to create regex");
    println!("Do we still match RuSt? {}", rust_regex.is_match("RuSt"));
}
