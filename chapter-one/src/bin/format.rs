/// `format!`包括三个部分
/// * 格式化字符串
/// * 格式化参数--花括号
/// * 实参 传入的值
///
/// 对于命名参数来说，其它没有命名的参数必须写到命名参数前面！
///
/// 位置参数和普通的花括号混用，但是这样用不太好，不好读
/// 有很多格式化方式，例如`{:?}`可以打印 Debug 特质的类型，`{:.*}`支持两个参数，位数+浮点数，打印指定位数的浮点数
fn main() {
    let colour = "red";
    let favourite = format!("My favourite colour is {}", colour);
    println!("{}", favourite);

    // 可以加很多参数，按顺序放置
    let hello = "hello ";
    let world = "world!";
    let hello_world = format!("{}{}", hello, world);
    println!("{}", hello_world);

    // 支持任何实现了 Display 特质的类型
    let favourite_num = format!("My favourite number is {}", 42);
    println!("{}", favourite_num);

    // 如果要重复添加同一个参数，在花括号中使用参数的位置
    let duck_duck_goose = format!("{0}, {0}, {0}, {1}", "duck", "goose");
    println!("{}", duck_duck_goose);

    // 可以为参数命名
    let introduction = format!(
        "My name is {surname}, {forename} {surname}",
        surname = "Bond",
        forename = "James"
    );
    println!("{}", introduction);
}
