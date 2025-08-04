use std::ops::Deref;

/// 我们可以创建一个结构体，给用户对资源和功能的临时访问能力，当用户使用完成后再自动回收它们。这叫做RAII，表示 **Resource Acquisition Is Initialization**
///
/// 对于RAII来说实现Deref不是必须的，但是很有用
///
/// 本例就像 RwLock 和 Mutex 的实现类似，xxxHandle不应该包含具体的 lock 和 unlock 实现，而是应该将保存的对应特殊OS（正在运行的）实现的调用传入，它只应该表示一个通用的接口，本例中就是表示那些支持的OS的拥有lock 和unlock资源能力的功能
fn main() {
    let foo = SomeOsFunctionality::new("Hello world");
    {
        let bar = foo.lock();
        println!("The string behind foo is {} characters long", bar.len());
        // foo 自动被解锁
    }
    // foo 已经被解锁
}

// 表示一个低级别的，靠近OS的特性，需要在访问前后进行锁定和解锁，通常调用方式是unsafe的
// 会操作一些数据，我们假设这个特性会锁一些操作系统的资源，并且之后需要解锁
struct SomeOsSpecificFunctionalityHandle;

// 一个安全的wrapper，需要保存一些数据
struct SomeOsFunctionality<T> {
    data: T,
    // 和OS交互的低级结构体通常对于 move 来说是不安全的
    // 但是我们不想限制用户不能移动这个安全的wrapper
    inner: Box<SomeOsSpecificFunctionalityHandle>,
}

// 一个RAII guard，使用 lock 函数创建，当它被清除时，自动解锁底层的资源，并且可以直接使用包含的数据 T
struct SomeOsFunctionalityGuard<'a, T: 'a> {
    lock: &'a SomeOsFunctionality<T>,
}

impl SomeOsSpecificFunctionalityHandle {
    unsafe fn lock(&self) {
        // unsafe low level code
    }

    unsafe fn unlock(&self) {
        // unsafe low level code
    }
}

impl<T> SomeOsFunctionality<T> {
    fn new(data: T) -> Self {
        let handle = SomeOsSpecificFunctionalityHandle;
        SomeOsFunctionality {
            data,
            inner: Box::new(handle),
        }
    }

    fn lock(&self) -> SomeOsFunctionalityGuard<T> {
        // unsafe 告诉编译器要特殊对待这个块，它禁止了借用检查器，可以做一些事情：解引用裸指针，调用unsafe函数等。同时编译器不提供任何保证，在以下两种情况可以使用unsafe
        // * 写直接和OS交互的代码，为unsafe部分创建安全的包裹器，就像本例
        // * 在特殊情况下，你确定你做的事情没有问题，只是编译器不理解（split_array）
        unsafe {
            self.inner.lock();
        }
        SomeOsFunctionalityGuard { lock: self }
    }
}

impl<'a, T> Drop for SomeOsFunctionalityGuard<'a, T> {
    fn drop(&mut self) {
        println!("system resource was been dropped");
        unsafe {
            self.lock.inner.unlock();
        }
    }
}

impl<'a, T> Deref for SomeOsFunctionalityGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.lock.data
    }
}
