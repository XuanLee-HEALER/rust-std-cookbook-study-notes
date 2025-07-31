use slab::{Slab, VacantEntry};

/// slab和vector很像，一个典型区别是：你不能选择索引。插入数据时，得到一个访问数据的索引，即key
/// 存储这个key是你的责任，否则得到这个数据的唯一方式是遍历
/// 相比于HashMap，你不需要提供任何可以哈希的对象作为key
///
/// 这种结构在连接池中很常用，将几个统一使用的资源放到slab中
/// 下面例子中限制访问资源的数量也是一个经典用例，你只需要限制当前访问资源的人数
///
/// 一般使用with_capacity来创建slab而不是new
///
/// slab背后是Vec<Entry>，Entry表示一个可能有或者没有的值，所以这些Entry会占用vector中的位置
/// 为了快速占用vacant spot，slab对于所有的vacant entries维持了一个链表
fn main() {
    // 一个slab可以作为有限大小的buffer，应该使用提前定义的容量
    const CAPACITY: usize = 1024;
    let mut slab = Slab::with_capacity(CAPACITY);

    // 你不能直接用索引或者搜索来访问slab，而是每个插入都会返回一个key
    // 需要用这个key来访问
    let hello_key = slab.insert("hello");
    let world_key = slab.insert("world");

    println!("hello_key -> '{}'", slab[hello_key]);
    println!("world_key -> '{}'", slab[world_key]);

    let data_key = {
        let entry = slab.vacant_entry();
        fill_some_data(entry)
    };
    println!("data_key -> '{}'", slab[data_key]);

    for (key, val) in &slab {
        println!("{} -> {}", key, val);
    }

    // 如果要让slab保持在固定的容量，需要在插入前检查长度
    // 可以将这个操作放到一个函数中，返回 Result 或者 Option
    if slab.len() != slab.capacity() {
        slab.insert("the slab is not at capacity yet");
    }
}

fn fill_some_data(entry: VacantEntry<&str>) -> usize {
    // insert会消耗这个entry，所以先获取key
    let data = "Some data";
    let key = entry.key();
    entry.insert(data);
    key
}
