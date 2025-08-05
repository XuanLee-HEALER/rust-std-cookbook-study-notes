use std::{
    sync::{Arc, RwLock},
    thread,
};

/// RwLock 等同于并行版本的 RefCell。区别是 RefCell 会在Rust的所有权规则被破坏时崩溃。RwLock只是阻塞当前线程直到这个破坏过程结束
/// read 和 write 会返回一个 Result，报告线程是否有毒（poisoned），不同的锁这个词的含义不同。RwLock中，表示锁住资源的线程崩溃了。遇到这种情况通常可以 panic，因为panic代表出现了严重错误
///
/// RwLock不是并发问题的银弹。存在死锁情况。避免此问题的方法是时刻记住这一点，并且在创建互相依赖的过程时想到它
/// 另一个锁时Mutex，它将所有的访问进程看作一个writer，所以两个线程永远不能同时处理同一个资源，即使它们不修改资源
fn main() {
    let resource = Arc::new(RwLock::new("Hello World!".to_string()));
    let reader_a = {
        let resource = Arc::clone(&resource);
        thread::spawn(move || {
            for _ in 0..40 {
                let resource = resource
                    .read()
                    .expect("Failed to lock resource for reading");
                println!("Reader A says: {}", resource);
            }
        })
    };

    let reader_b = {
        let resource = Arc::clone(&resource);
        thread::spawn(move || {
            for _ in 0..40 {
                let resource = resource
                    .read()
                    .expect("Failed to lock resource for reading");
                println!("Reader B says: {}", resource);
            }
        })
    };

    let writer = {
        let resource = Arc::clone(&resource);
        thread::spawn(move || {
            for _ in 0..10 {
                let mut resource = resource
                    .write()
                    .expect("Failed to lock resource for writing");
                resource.push('!');
            }
        })
    };

    reader_a.join().expect("Reader A panicked");
    reader_b.join().expect("Reader B panicked");
    writer.join().expect("Writer panicked");
}
