/// 迭代器非常重要，无论你干什么，使用什么库，都会用到它。这里所有的操作都能在任何集合上和所有实现了iterator特质的类型上使用
/// 之所以range是limited，是因为range类型实现了Step，但是char没有，所以不能将 'a'..'D' 作为迭代器使用
///
/// 在使用nth时，要注意两件事
/// * 它是通过遍历来访问到那个项，所以是O(N)的复杂度，如果可以，使用集合自带的 访问方法，例如get
/// * 它会消耗迭代器，直到那个项，所以两个相同参数的nth调用会得到不同的结果
///
/// 对于zip来说，如果两个迭代器的数量不同，它会忽略多出来的那些
///
/// iter()创建的迭代器会借用项，如果你要创建一个消耗项的迭代器，使用into_iter()
fn main() {
    let names = vec!["Joe", "Miranda", "Alice"];

    // 几乎所有的集合都实现了 .iter()
    let mut iter = names.iter();

    // string本身不能被迭代，但是它的字符可以
    let mut alphabet = "ABCDEFGHIGKLMNOPQRSTUVWXYZ".chars();

    // range 也是 （有限的）迭代器
    let nums = 0..10;

    // 创建一个无限迭代器
    let all_nums = 0..;

    for num in nums {
        print!("{} ", num);
    }
    // nums不能再使用了
    println!();

    for (idx, letter) in "abc".chars().enumerate() {
        println!("#{}. letter in the alphabet: {}", idx + 1, letter)
    }

    if let Some(name) = iter.next() {
        println!("First name: {}", name);
    }
    if let Some(name) = iter.next() {
        println!("Second name: {}", name)
    }
    if let Some(name) = iter.next() {
        println!("Third name: {}", name)
    }
    if iter.next().is_none() {
        println!("No names left")
    }

    // 直接访问迭代器中某个元素，会消耗掉它之前的所有元素
    let letter = alphabet.nth(3);
    if let Some(letter) = letter {
        println!("the fourth letter in the alphabet is: {}", letter)
    }

    let current_first = alphabet.nth(0);
    if let Some(current_first) = current_first {
        println!(
            "The first item in the iterator is currently: {}",
            current_first
        )
    }
    let current_first = alphabet.nth(0);
    if let Some(current_first) = current_first {
        println!(
            "The first item in the iterator is currently: {}",
            current_first
        )
    }

    // 访问迭代器最后一个元素，这会消耗整个迭代器
    let last_letter = alphabet.last();
    if let Some(last_letter) = last_letter {
        println!("The last letter of the alphabet is: {}", last_letter)
    }

    let nums: Vec<_> = (1..10).collect();
    println!("nums: {:?}", nums);
    let nums = (1..10).collect::<Vec<_>>();
    println!("nums: {:?}", nums);

    // 获取前n个项，通常用来将无限流转换为有限流
    let nums: Vec<_> = all_nums.take(5).collect();
    println!("The first five numbers are: {:?}", nums);

    let nums: Vec<_> = (0..11).skip(2).collect();
    println!("The last 8 letters in a range from zero to 10: {:?}", nums);

    // 获取符合predicate和跳过符合predicate的元素
    let nums: Vec<_> = (0..).take_while(|x| x * x < 50).collect();
    println!(
        "All positive numbers that are less than 50 when squared: {:?}",
        nums
    );

    let names = ["Alfred", "Andy", "Jose", "Luke"];
    let names: Vec<_> = names.iter().skip_while(|x| x.starts_with('A')).collect();
    println!("Names that don't start with 'A': {:?}", names);

    let countries = [
        "U.S.A.", "Germany", "France", "Italy", "India", "Pakistan", "Burma",
    ];
    let countries_with_i: Vec<_> = countries
        .iter()
        .filter(|country| country.contains('i'))
        .collect();
    println!(
        "Countries containing the letter 'i': {:?}",
        countries_with_i
    );

    // 找到满足条件的第一个项
    if let Some(country) = countries.iter().find(|country| country.starts_with('I')) {
        println!("First country starting with the letter 'I': {}", country);
    }

    // 获取索引而不是它的项
    if let Some(pos) = countries
        .iter()
        .position(|country| country.starts_with('I'))
    {
        println!("It's index is: {}", pos)
    }

    let are_any = countries.iter().any(|country| country.len() == 5);
    println!(
        "Is there at least one country that has exactly five letters? {}",
        are_any
    );

    let are_all = countries.iter().all(|country| country.len() == 5);
    println!("Do all countries have exactly five letters? {}", are_all);

    //数字操作
    let sum: i32 = (1..11).sum();
    let product: i32 = (1..11).product();
    println!(
        "When operation on the first ten positive numbers\n\
their sum is {} and\n\
their product is {}.",
        sum, product
    );

    let max = (1..11).max();
    let min = (1..11).min();
    if let Some(max) = max {
        println!("They have a highest number, and it is {}", max)
    }
    if let Some(min) = min {
        println!("They have a smallest number, and it is {}", min)
    }

    // 将一个迭代器和它自己组合，可以无限迭代
    let some_numbers: Vec<_> = (1..4).cycle().take(10).collect();
    // 1 2 3 1 2 3 1 2 3 1
    println!("some_numbers: {:?}", some_numbers);

    // 连接两个迭代器，将一个放到另一个后面
    let some_numbers: Vec<_> = (1..4).chain(10..14).collect();
    println!("some_numbers: {:?}", some_numbers);

    // zip两个迭代器，将它们的每个项组织到一起
    let swiss_post_codes = [8957, 5000, 5034];
    let swiss_towns = ["Spreitenbach", "Aarau", "Suhr"];
    let zipped: Vec<_> = swiss_post_codes.iter().zip(swiss_towns.iter()).collect();
    println!("zipped: {:?}", zipped);

    // zip是懒的，所以你可以zip两个无限迭代器
    let zipped: Vec<_> = (b'A'..)
        .zip(1..)
        .take(10)
        .map(|(ch, num)| (ch as char, num))
        .collect();
    println!("zipped: {:?}", zipped);

    let numbers_as_strings: Vec<_> = (1..11).map(|x| x.to_string()).collect();
    println!("numbers_as_strings: {:?}", numbers_as_strings);

    println!("First ten squares:");
    (1..11).for_each(|x| print!("{} ", x));
    println!();

    let squares: Vec<_> = (1..50)
        .filter_map(|x| if x % 3 == 0 { Some(x * x) } else { None })
        .collect();
    println!(
        "Squares of all numbers under 50 that are divisibly by 3: {:?}",
        squares
    );

    let alphabet: Vec<_> = (b'A'..b'z' + 1)
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect();
    println!("alphabet: {:?}", alphabet)
}
