use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

/// Rust的原子操作有5种内存顺序，从最弱到最强排序
/// 1. Relaxed 只保证单个操作是原子的，不提供跨线程的同步保证，不阻止编译器和CPU重排代码
/// 2. Acquire 加载屏障。加载（load）的同步保证，确保这个 Acquire 的操作之后所有内存操作不能被重排到之前
/// 通常和Release配合使用，在Release之前做的事情，Acquire之后的操作都能看到
/// 3. Release 存储屏障。存储（store）的同步保证，确保这个 Release 之前的所有内存操作不能重排到之后
/// 4. AcqRel Acquire-Release，加载-修改-存储的同步保证。进入时可以看到别人之前做的事，离开时，确保你自己做的事都可见
/// 5. SeqCst Sequentially Consistent，所有操作的同步保证，确保所有线程在任何时间点看到的内存操作顺序都是完全一致的，阻止编译器和CPU重新排序所有 SeqCst 操作
fn main() {}

pub struct NaiveMutex<T> {
    locked: AtomicBool,
    // UnsafeCell是每个内部可变容器的底层结构
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

    pub fn lock(&self) -> NaiveMutexGuard<T> {
        // 自旋锁
        while let Ok(b) =
            self.locked
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            && !b
        {}
        NaiveMutexGuard { naive_mutex: self }
    }
}

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
