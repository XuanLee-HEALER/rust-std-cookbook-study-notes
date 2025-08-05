#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }
}

/// rayon::join 接收两个闭包，并行运行它们，返回一个元组包含两个闭包的返回值
///
/// Intel CPU可以使用超线程技术，意味着一个程序只使用物理核心的加法器资源，同时另一个程序可以使用虚拟核心的浮点数计算资源，但是它们利用的仍然是同一个物理核心，并发而不是并行
///
/// 如果不是两件事必须并行执行，你首先应该查看当前是否还有空闲的核心，rayon::join帮你做了这件事。它只有在值得做这件事的时候才会并行运行两个闭包。如果要手动做这件事，可以使用 num_cpus crate
/// 对于并行迭代器，如果处理的元素很少，那么会自动放弃并发
///
/// rayon的底层机制是 work stealing，对于 rayon::join(a, b) 当前线程会立即开始 a ，然后将 b 放到队列中。当核心空闲，rayon会让它处理队列中下一个元素。新线程会从其它线程偷（steal）任务，如果a完成，猪线程会查看队列，尝试偷任务
fn main() {
    let rect = Rectangle {
        height: 30,
        width: 20,
    };

    let (area, perimeter) = rayon::join(|| rect.area(), || rect.perimeter());
    println!("{:?}", rect);
    println!("area: {}", area);
    println!("perimeter: {}", perimeter);

    let fib = fibonacci(6);
    println!("The sixth number in the fibonacci sequence is {}", fib)
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else {
        let (a, b) = rayon::join(|| fibonacci(n - 1), || fibonacci(n - 2));
        a + b
    }
}
