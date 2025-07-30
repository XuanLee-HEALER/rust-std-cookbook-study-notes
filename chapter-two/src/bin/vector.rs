/// vector
/// * vector是最重要的集合
/// * 很多核心原则、例如预分配，都应用到了其它的集合
/// * 包括很多应用于切片的方法，同样被其它集合使用
///
/// 因为 vec! 很好用，第三方 maplit crate提供了HashMap、Set、BTreeMap、Set的宏
/// vector 的 push和pop 都是移动**最后一个元素**，由于内存布局，操作时间复杂度O(1)，可以构成一个FILO的栈
///
/// insert、remove、swap可以修改元素
/// 虽然 get/get_mut 返回的是Option，但是由于数组越界没什么需要处理的，所以提供了索引语法替代 unwrap
///
/// drain会消耗vector中的元素，对于要重复利用原来的vector，这个方法是很有用的
///
/// # 实现
/// 存储在堆上的连续块，关键字是**连续**，意味着内存是缓存友好的。vector会分配额外的内存以便扩展。⚠️开始插入大量的数据会导致整个栈移动
/// 分配的额外空间是为了减少重分配。额外的空间在缩小vector时不会消失。对于有内存限制的平台，需要利用 shrink_to_fit 将容量尽可能缩小到和长度相同，但是还是会留有一些预分配空间
/// 另一个提高性能的方式是使用 swap_remove ，如果你不在意vector元素的顺序，这种方式比直接remove要节省时间，因为后面的元素不会依次前移，并且交换内存上的两个位置现在是很便宜的操作
fn main() {
    let fruits = vec!["apple", "tomato", "pear"];
    println!("fruits: {:?}", fruits);

    let mut fruits = Vec::new();
    fruits.push("apple");
    fruits.push("tomato");
    fruits.push("pear");
    println!("fruits: {:?}", fruits);

    let last = fruits.pop();
    if let Some(last) = last {
        println!("Removed {} from: {:?}", last, fruits);
    }

    fruits.insert(1, "grape");
    println!("fruits after insertion: {:?}", fruits);

    fruits.swap(0, 1);
    println!("fruits after swap: {:?}", fruits);

    let first = fruits.first();
    if let Some(first) = first {
        println!("First fruit: {}", first);
    }

    let last = fruits.last();
    if let Some(last) = last {
        println!("Last fruit: {}", last);
    }

    let second = fruits.get(1);
    if let Some(second) = second {
        println!("Second fruit: {}", second);
    }

    // 没有边界检查
    let second = fruits[1];
    println!("Second fruit: {}", second);

    let bunch_of_zeroes = vec![0; 5];
    println!("bunch_of_zeroes: {:?}", bunch_of_zeroes);

    // 删除一个中间值，后面的值会移动到前面
    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.remove(1);
    println!("Removed {} from {:?}", second_num, nums);

    // 原地（in place）过滤操作
    let mut names = vec!["Aaron", "Felicia", "Alex", "Daniel"];
    // 留下符合条件的元素
    names.retain(|name| name.starts_with('A'));
    println!("Names starting with A: {:?}", names);

    println!("Does 'names' contain \"Alex\"? {}", names.contains(&"Alex"));

    // 去重，连续的重复值
    let mut nums = vec![1, 2, 2, 3, 4, 4, 4, 5];
    nums.dedup();
    println!("Deduped, pre-sorted nums: {:?}", nums);

    let mut nums = vec![2, 1, 4, 2, 3, 5, 1, 2];
    nums.dedup();
    println!("Deduped, unsorted nums: {:?}", nums);

    nums.sort();
    println!("Manually sorted nums: {:?}", nums);
    nums.dedup();
    println!("Deduped, sorted nums: {:?}", nums);

    nums.reverse();
    println!("nums after being reversed: {:?}", nums);

    // 在一个range上创建一个消费迭代器
    let mut alphabet = vec!['a', 'b', 'c'];
    print!("The first two letters of the alphabet are: ");
    for letter in alphabet.drain(..2) {
        print!("{} ", letter);
    }
    println!();
    println!("alphabet after being drained: {:?}", alphabet);

    let mut fridge = vec!["Beer", "Leftovers", "Mayonaise"];
    println!("Is the fridge empty? {}", fridge.is_empty());

    fridge.clear();
    println!("Is the fridge empty? {}", fridge.is_empty());

    let mut colors = vec!["red", "green", "blue", "yellow"];
    println!("colors before splitting: {:?}", colors);
    let half = colors.len() / 2;
    let mut second_half = colors.split_off(half);
    println!("colors after splitting: {:?}", colors);
    println!("second_half: {:?}", second_half);

    colors.append(&mut second_half);
    println!("colors after appending: {:?}", colors);
    // 上面的操作会清空第二个vector
    println!("second_half after appending: {:?}", second_half);

    let mut stuff = vec!["1", "2", "3", "4", "5", "6"];
    println!("Original stuff: {:?}", stuff);
    let stuff_to_insert = vec!["a", "b", "c"];
    let removed_stuff: Vec<_> = stuff.splice(1..4, stuff_to_insert).collect();
    println!("Spliced stuff: {:?}", stuff);
    println!("Removed stuff: {:?}", removed_stuff);

    // 大数据集，优化vector的性能
    let mut large_vec: Vec<i32> = Vec::with_capacity(1_000_000);
    println!("large_vec after creation:");
    println!("len:\t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());

    // 缩小数据集到长度
    large_vec.shrink_to_fit();
    println!("large_vec after shrinking:");
    println!("len:\t\t{}", large_vec.len());
    println!("capacity:\t{}", large_vec.capacity());

    // 移除某个项，用最后一个填补
    let mut nums = vec![1, 2, 3, 4];
    // O(1)的时间复杂度
    let second_num = nums.swap_remove(1);
    println!("Removed {} from {:?}", second_num, nums);
}
