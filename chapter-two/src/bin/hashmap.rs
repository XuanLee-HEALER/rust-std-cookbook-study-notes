use std::collections::HashMap;

/// HashMap表示一个集合，将一种类型的数据映射到另一种上。通过insert来实现这件事，如果key已经有值，就会覆盖，所以返回的是Option，如果已经有值就会返回旧值
/// HashMap允许你做三件事
/// * 获取所有值的引用
/// * 获取所有值的可变引用
/// * 获取所有键的引用
///
/// Entry API让你访问一个可能在或不在HashMap中的值。如果你想插入基于闭包的默认值，使用 or_insert_with
/// 另一种用法是和它的变体匹配：Occupied 或者 Vacant，效果和 get某个key相同
/// or_insert返回一个可变引用
///
/// 利用 with_capacity/shrink_to_fit/reserve来改善HashMap的性能
///
/// HashMap的实现：buffer保存值，以有序方式。还有一张表来存储指向表示它们代表的元素的桶，插入一个kv时
/// 1. 将值插入buffer
/// 2. key经过hash函数，成为一个索引
/// 3. table创建一个桶，表示这个索引指向某个值
///
/// Rust的hash算法没有生成唯一的索引，为了性能，使用了 Robin Hood bucket stealing
/// 使用with_hasher可以替换hash方法
fn main() {
    let mut tv_ratings = HashMap::new();
    tv_ratings.insert("The IT Crowd", 8);
    tv_ratings.insert("13 Reasons Why", 7);
    tv_ratings.insert("House of Cards", 9);
    tv_ratings.insert("Stranger Things", 8);
    tv_ratings.insert("Breaking Bad", 10);

    let contains_tv_show = tv_ratings.contains_key("House of Cards");
    println!("Dis we rate House of Cards? {}", contains_tv_show);
    let contains_tv_show = tv_ratings.contains_key("House");
    println!("Dis we rate House? {}", contains_tv_show);

    if let Some(rating) = tv_ratings.get("Breaking Bad") {
        println!("I rate Breaking Bad {} out of 10", rating)
    }

    // 插入一个值两次会覆盖
    let old_rating = tv_ratings.insert("13 Reason Why", 9);
    if let Some(old_rating) = old_rating {
        println!("13 Reason Why's old rating was {} out of 10", old_rating)
    }
    if let Some(rating) = tv_ratings.get("13 Reasons Why") {
        println!("But I changed my mind, it's now {} out of 10", rating)
    }

    let removed_value = tv_ratings.remove("The IT Crowd");
    if let Some(removed_value) = removed_value {
        println!("The removed series had a rating {}", removed_value)
    }

    println!("All ratings:");
    for (key, value) in &tv_ratings {
        println!("{}\t: {}", key, value)
    }

    println!("All ratings with 100 as a maximum:");
    for (key, value) in &mut tv_ratings {
        *value *= 10;
        println!("{}\t: {}", key, value)
    }

    for _ in tv_ratings {}
    // tv_ratings不能再次被使用

    // 预先分配内存来提高性能
    let mut age = HashMap::with_capacity(10);
    age.insert("Dory", 8);
    age.insert("Nemo", 5);
    age.insert("Merlin", 10);
    age.insert("Bruce", 9);

    println!("All names:");
    for name in age.keys() {
        println!("{}", name)
    }

    println!("All ages:");
    for age in age.values() {
        println!("{}", age)
    }

    println!("All ages in 10 years");
    for age in age.values_mut() {
        *age += 10;
        println!("{}", age)
    }

    // 使用entry API，如果在HashMap中不存在key，就分配一个默认值给这个key
    {
        let age_of_coral = age.entry("coral").or_insert(11);
        println!("age_of_coral: {}", age_of_coral);
    }

    let age_of_coral = age.entry("coral").or_insert(15);
    println!("age_of_coral: {}", age_of_coral)
}
