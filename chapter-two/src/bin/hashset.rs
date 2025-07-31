use std::collections::HashSet;

/// HashSet是一种HashMap，大部分接口都相似。主要区别是在HashMap中返回值的方法在HashSet中返回bool
/// 表示是否存在
///
/// 背后实现和HashMap类似，只有表的部分（桶和key值的关系）
fn main() {
    let mut books = HashSet::new();
    books.insert("Harry Potter and the Philosopher's Stone");
    books.insert("The Name of the Wind");
    books.insert("A Game of Thrones");

    // 返回是否是新元素
    let is_new = books.insert("The Lies of LockeLamora");
    if is_new {
        println!("We've just added a new book!")
    }

    let is_new = books.insert("A Game of Thrones");
    if !is_new {
        println!("Sorry, we already had that book store")
    }

    if !books.contains("The Doors of Stone") {
        println!("We sadly don't have that book yet")
    }

    let was_removed = books.remove("The Darkness that comes before");
    if !was_removed {
        println!("Couldn't remove book; We didn't have it to begin with")
    }

    let was_removed = books.remove("Harry Potter and the Philosopher's Stone");
    if was_removed {
        println!("oops, we lost a book")
    }

    // 比较不同的HashSet
    let one_to_five: HashSet<_> = (1..=5).collect();
    let five_to_ten: HashSet<_> = (5..=10).collect();
    let one_to_ten: HashSet<_> = (1..=10).collect();
    let three_to_eight: HashSet<_> = (3..=8).collect();

    // 两个集合没有交集
    let is_disjoint = one_to_five.is_disjoint(&five_to_ten);
    println!(
        "is {:?} disjoint from {:?}?: {}",
        one_to_five, five_to_ten, is_disjoint
    );
    let is_disjoint = one_to_five.is_disjoint(&three_to_eight);
    println!(
        "is {:?} disjoint from {:?}?: {}",
        one_to_five, three_to_eight, is_disjoint
    );

    // 一个集合是否完全包含于另一个集合
    let is_subset = one_to_five.is_subset(&five_to_ten);
    println!(
        "is {:?} a subset of {:?}?: {}",
        one_to_five, five_to_ten, is_subset
    );
    let is_subset = one_to_five.is_subset(&one_to_ten);
    println!(
        "is {:?} a subset of {:?}?: {}",
        one_to_five, one_to_ten, is_subset
    );

    // 一个集合是否完全包含另一个集合
    let is_superset = three_to_eight.is_superset(&five_to_ten);
    println!(
        "is {:?} a superset of {:?}?: {}",
        three_to_eight, five_to_ten, is_superset
    );
    let is_superset = one_to_ten.is_superset(&five_to_ten);
    println!(
        "is {:?} a superset of {:?}?: {}",
        one_to_ten, five_to_ten, is_superset
    );

    // 两个集合的差集
    let difference = one_to_five.difference(&three_to_eight);
    println!(
        "The difference between {:?} and {:?} is {:?}",
        one_to_five, three_to_eight, difference
    );

    // 两个集合的全集减去两个集合的交集
    let symmetric_difference = one_to_five.symmetric_difference(&three_to_eight);
    println!(
        "The symmetric difference between {:?} and {:?} is {:?}",
        one_to_five, three_to_eight, symmetric_difference
    );

    // 两个集合的交集
    let intersection = one_to_five.intersection(&three_to_eight);
    println!(
        "The intersection difference between {:?} and {:?} is {:?}",
        one_to_five, three_to_eight, intersection
    );

    // 两个集合的全集
    let r#union = one_to_five.union(&three_to_eight);
    println!(
        "The union difference between {:?} and {:?} is {:?}",
        one_to_five, three_to_eight, r#union
    );
}
