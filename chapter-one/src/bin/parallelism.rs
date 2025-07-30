use std::thread;

/// 你无法控制线程完成工作的顺序，有时OS spawn和管理新线程并不值得
fn main() {
    // 通过 thread::spawn 创建一个新线程，会开始执行提供的闭包，返回一个 JoinHandle
    let child = thread::spawn(|| println!("Hello from a new thread!"));
    println!("Hello from the main thread");
    // 将子线程 join 到主线程的意思是 主线程需要等到子线程执行完成
    // 如果不join线程，就无法保证该线程是否完成任务。有时让线程执行永远不会完成的任务是有效的，例如监听某个端口
    child.join().expect("Failed to join the child thread");

    let sum = parallel_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("The sum of the numbers 1 to 10 is {}", sum);
}

fn parallel_sum(range: &[i32]) -> i32 {
    const NUM_THREAD: usize = 4;
    if range.len() < NUM_THREAD {
        sum_bucket(range)
    } else {
        let bucket_size = range.len() / NUM_THREAD;
        let mut count = 0;
        let mut threads = vec![];
        while count + bucket_size < range.len() {
            // 这里的to_vec是我们将range的数据拷贝到bucket
            // 如果直接在新线程中使用引用是有问题的，因为range 仅在 parallel_sum有效，但是spawn的新线程允许存活时间超过父线程
            let bucket = range[count..count + bucket_size].to_vec();
            // let bucket = &range[count..count + bucket_size];
            let thread = thread::Builder::new()
                // 如果OS支持命名线程，线程名会在panic时展示出来
                .name("calculation".to_string())
                .spawn(move || sum_bucket(&bucket))
                .expect("Failed to create the thread");
            threads.push(thread);
            count += bucket_size
        }

        let mut sum = sum_bucket(&range[count..]);

        for thread in threads {
            // join一个线程会返回一个 Result，包含闭包的返回值和可能的错误
            sum += thread.join().expect("Failed to join thread");
        }
        sum
    }
}

fn sum_bucket(range: &[i32]) -> i32 {
    // panic!("random error");
    range.iter().sum()
}
