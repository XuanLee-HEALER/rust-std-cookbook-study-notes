use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::{
        Arc,
        atomic::{ATOMIC_USIZE_INIT, AtomicBool, AtomicUsize, Ordering},
    },
    thread,
};

/// Rust的原子操作有5种内存顺序，从最弱到最强排序
/// 1. Relaxed 只保证单个操作是原子的，不提供跨线程的同步保证，不阻止编译器和CPU重排代码
/// 2. Acquire 加载屏障。加载（load）的同步保证，确保这个 Acquire 的操作之后所有内存操作不能被重排到之前
/// 通常和Release配合使用，在Release之前做的事情，Acquire之后的操作都能看到
/// 3. Release 存储屏障。存储（store）的同步保证，确保这个 Release 之前的所有内存操作不能重排到之后
/// 4. AcqRel Acquire-Release，加载-修改-存储的同步保证。进入时可以看到别人之前做的事，离开时，确保你自己做的事都可见
/// 5. SeqCst Sequentially Consistent，所有操作的同步保证，确保所有线程在任何时间点看到的内存操作顺序都是完全一致的，阻止编译器和CPU重新排序所有 SeqCst 操作
///
/// 在atomic模块下有4种（写书时，现在多了i8、i16等）基本类型，bool、usize、isize、pointer，这里不展开pointer，因为这种类型只有在处理和其它语言写的程序的接口时会使用
/// 基本类型的 atomic 版本和普通类型工作方式相同，一个重要区别是，在并行环境中对它们的使用是完全定义的（well-defined）。它们的方法都需要接收一个 atomic::Ordering 类型参数。表示使用的低级并发策略，本节只使用了 SeqCst，如果用这种策略存储和修改值，其它线程可以看到它被写入之后的值，就像一个线程运行在另一个线程之后
///
/// atomic的真实使用，通常在于实现我们自己的同步原语（Mutex）
fn main() {
    // 通常通过从全局常量复制生成
    let some_number = AtomicUsize::new(0);
    // 推荐使用 new，和上一条语句等价
    // let some_number = ATOMIC_USIZE_INIT;

    // load 获取这个atomic的当前值，Ordering告诉编译器如何处理和其它线程的交互
    let curr_val = some_number.load(Ordering::SeqCst);
    println!("The current value of some_number is {}", curr_val);

    // store 设置变量
    some_number.store(123, Ordering::SeqCst);
    let curr_val = some_number.load(Ordering::SeqCst);
    println!("The current value of some_number is {}", curr_val);

    // swap 设置变量，并返回旧值
    let old_val = some_number.swap(12_345, Ordering::SeqCst);
    let curr_val = some_number.load(Ordering::SeqCst);
    println!("The old value of some_number was {}", old_val);
    println!("The current value of some_number is {}", curr_val);

    // compare_and_swap 如果当前值等于第一个参数，就交换为新值，总是返回旧值
    let comparison = 12_345;
    let new_val = 6_789;
    let old_val = some_number
        .compare_exchange(comparison, new_val, Ordering::SeqCst, Ordering::SeqCst)
        .unwrap();
    if old_val == comparison {
        println!("The value has been updated")
    }

    // 上面的代码和下面的顺序代码等效
    let mut some_normal_number = 12_345;
    let old_val = some_normal_number;
    if old_val == comparison {
        some_normal_number = new_val;
        println!("The value has been updated sequentially")
    }

    // fetch_xxx 使用旧值做计算，然后返回旧值
    let old_val_one = some_number.fetch_add(12, Ordering::SeqCst);
    let old_val_two = some_number.fetch_sub(24, Ordering::SeqCst);
    let curr_val = some_number.load(Ordering::SeqCst);
    println!(
        "some_number was first {}, then {} and is now {}",
        old_val_one, old_val_two, curr_val
    );

    // fetch_or 可以执行变量和另一个参数执行 || 操作，返回旧值
    let some_bool = AtomicBool::new(false);
    let old_val = some_bool.fetch_or(true, Ordering::SeqCst);
    let curr_val = some_bool.load(Ordering::SeqCst);
    println!("({} || true) is {}", old_val, curr_val);

    let naive_mutex = Arc::new(NaiveMutex::new(1));
    let updater = {
        let naive_mutex = Arc::clone(&naive_mutex);
        thread::spawn(move || {
            let mut val = naive_mutex.lock();
            *val = 2;
        })
    };

    let printer = {
        let naive_mutex = Arc::clone(&naive_mutex);
        thread::spawn(move || {
            let val = naive_mutex.lock();
            println!("The value in the naive mutex is: {}", *val)
        })
    };

    updater.join().expect("The updater thread panicked");
    printer.join().expect("The printer thread panicked");
}

pub struct NaiveMutex<T> {
    // 追踪当前的mutex是否可用
    locked: AtomicBool,
    // UnsafeCell是每个内部可变容器的底层结构
    // 不保证任何借用安全性，需要调用者保证对它的操作的正确性
    data: UnsafeCell<T>,
}

pub struct NaiveMutexGuard<'a, T: 'a> {
    naive_mutex: &'a NaiveMutex<T>,
}

impl<T> NaiveMutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    // 这不是 std::sync::Mutex 实现的方式，操作系统原生支持，Mutex使用的是OS的mutex handle而不是自定义逻辑，Windows实现使用了 SRWLocks，要比原生的Mutex快
    pub fn lock(&self) -> NaiveMutexGuard<T> {
        // 自旋锁
        // 如果 self.locked 是 false，设为true，无论如何会返回旧值
        // 如果返回值是true，说明mutex正在被锁，什么都不做，这就是一个自旋锁结构
        while self
            .locked
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .unwrap()
        {}
        // RAII guard
        NaiveMutexGuard { naive_mutex: self }
    }
}

// 如果一个类型T实现了Send，表示它作为值在线程之间传递是安全的，几乎所有的类型都实现了Send
// 如果一个类型是Sync的，它告诉编译器作为这个类型的值作为引用在线程之间共享是安全的（以同步方式），因为我们只允许同一时间只有一个资源的访问者，这也是我们为所有Send去实现了Sync
unsafe impl<T: Send> Sync for NaiveMutex<T> {}

impl<'a, T> Drop for NaiveMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.naive_mutex.locked.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for NaiveMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.naive_mutex.data.get() }
    }
}

impl<'a, T> DerefMut for NaiveMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.naive_mutex.data.get() }
    }
}
