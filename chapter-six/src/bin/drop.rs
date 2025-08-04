use std::fmt::Debug;

struct CustomSmartPointer<D>
where
    D: Debug,
{
    data: D,
}

impl<D> CustomSmartPointer<D>
where
    D: Debug,
{
    fn new(data: D) -> Self {
        Self { data }
    }
}

impl<D> Drop for CustomSmartPointer<D>
where
    D: Debug,
{
    fn drop(&mut self) {
        // 变量被清理时自动调用
        println!("Dropping CustomSmartPointer with data `{:?}`", self.data)
    }
}

fn main() {
    // 域中最先声明的变量最后被清理
    let a = CustomSmartPointer::new("A");
    let b = CustomSmartPointer::new("B");
    let c = CustomSmartPointer::new("C");
    let d = CustomSmartPointer::new("D");

    // 不能直接调用 drop 方法
    // c.drop();

    // 正确写法
    // 这就是drop的实现，只是获取所有权 pub fn drop<T>(_x: T) {}
    drop(c);
}
