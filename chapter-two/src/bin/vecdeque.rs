use std::collections::VecDeque;

/// 内部，VecDeque使用环形缓冲区（ring buffer）实现，也叫circle buffer。末尾连接开头
/// 这个结构会分配一块连续内存，和Vec不同，它不会分配额外的空间。当你移除第一个元素，它不会移动所有元素往左，而是让那个元素空着，如果你在开头插入元素，这个元素会占之前的空间
/// 在使用push_back的时候，如果内存块后面没有空间而前面还有一些空间，VecDeque会利用前面的空间来保存这个元素，而内部实现保证了API的遍历顺序
/// 当容量用完，VecDeque会扩容并将所有元素移动到新的内存区域
fn main() {
    // VecDeque最好被看作FIFO队列
    let mut orders = VecDeque::new();
    println!("A guest ordered oysters!");
    orders.push_back("oysters");

    println!("A guest ordered fish and chips!");
    orders.push_back("fish and chips");

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared);
    }

    println!("A guest ordered mozarella sticks!");
    orders.push_back("mozarella sticks");

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared)
    }

    println!("A guest ordered onion rings");
    orders.push_back("onion rings");

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared)
    }

    let prepared = orders.pop_front();
    if let Some(prepared) = prepared {
        println!("{} are ready", prepared)
    }

    let mut sentence = VecDeque::new();
    sentence.push_back("a");
    sentence.push_front("had");
    sentence.push_back("little");
    sentence.push_front("Mary");
    sentence.push_back("Lamb");
    println!("sentence: {:?}", sentence);

    sentence.pop_front();
    sentence.push_front("Jimmy");
    sentence.pop_back();
    sentence.push_back("Cat");
    println!("sentence: {:?}", sentence);

    // VecDeque 大部分方法和 Vec相同，有一些单独的方法
    let mut some_queue = VecDeque::with_capacity(5);
    some_queue.push_back("A");
    some_queue.push_back("B");
    some_queue.push_back("C");
    some_queue.push_back("D");
    some_queue.push_back("E");
    println!("some_queue: {:?}", some_queue);

    // 和Vec的swap_remove等效
    some_queue.swap_remove_back(2);
    println!("some_queue after swap_remove_back: {:?}", some_queue);

    // 和swap_remove效果相同，但是和第一个元素交换
    some_queue.swap_remove_front(2);
    println!("some_queue after swap_remove_front: {:?}", some_queue);
}
