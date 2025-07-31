use std::{
    ops::{Deref, Mul},
    thread::sleep,
    time::Duration,
};

/// next（Iterator trait）方法获取下一个项，如果迭代器没有项了，就返回None，这是一个将一种算法转换为一个迭代器的例子
///
/// SquaredVec更像是一个集合类型。Mul + Copy限制要求用户想保存的值必须可以相乘，可以被复制
/// 我们也可以为它实现Iterator特质，但是更简单的方式是让我们的结构体可以隐式转换为 [T] 类型
///
/// 如果你需要在迭代器中执行很多复杂逻辑，想将它和你的集合分开，可以实现IntoIterator，这可以让你单独返回一个用于迭代的结构，它本身提供Iterator特质
fn main() {
    let fib: Vec<_> = fibonacci().take(10).collect();
    println!("First 10 numbers of the fibonacci sequence: {:?}", fib);

    let mut squared_vec = SquaredVec::new();
    squared_vec.push(1);
    squared_vec.push(2);
    squared_vec.push(3);
    squared_vec.push(4);
    for (index, num) in squared_vec.iter().enumerate() {
        println!("{}^2 is {}", index + 1, num)
    }

    let time_seq: TimeSeq<i32> = TimeSeq { _list: Vec::new() };
    for v in time_seq {
        println!("{}", v);
        sleep(Duration::from_secs(1));
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let old = self.curr;
        self.curr = self.next;
        self.next += old;
        Some(old)
    }
}

struct SquaredVec<T>
where
    T: Mul + Copy,
{
    vec: Vec<T::Output>,
}

impl<T> SquaredVec<T>
where
    T: Mul + Copy,
{
    fn new() -> Self {
        SquaredVec { vec: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.vec.push(item * item);
    }
}

// 使用类似集合的结构创建一个迭代器
// 最好可以转换成底层类型的切片，相当于把SquaredVec类型变成一个指针
// 这样做可以自动实现很多方法（使用底层结构的方法），为了之后修改实现更灵活
impl<T> Deref for SquaredVec<T>
where
    T: Mul + Copy,
{
    type Target = Vec<T::Output>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

struct TimeSeq<T> {
    _list: Vec<T>,
}

impl<T> IntoIterator for TimeSeq<T> {
    type Item = u32;

    type IntoIter = Fibonacci;

    fn into_iter(self) -> Self::IntoIter {
        fibonacci()
    }
}
