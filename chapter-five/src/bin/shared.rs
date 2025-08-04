use std::rc::Rc;

struct Kid {
    ball: Rc<Ball>,
}

struct Ball;

/// `Rc`的关键在于有多少对象同时拥有它，这个内部计数器。每次 Rc 被克隆，就会加1，每次一个克隆离开域，就会减1。当这个计数器到达0，Rc维护的对象就会被销毁
/// 这个简单规则要表达的效果是，Rc背后的资源只会被销毁一次，当它不再被使用时。由于只是计数，所以其运行时开销非常小。这种延迟删除的效果对于那些需要在不同对象中共享的资源很理想
///
/// 在一种特殊情况下，引用计数可能导致内存泄漏（memory leak），即引用的资源永远无法被清理。当两个对象都拥有对方的Rc时会发生，由于循环依赖，导致两个对象都无法被清理。解决方法是使用 `Weak` 来替代 `Rc` ，它包含一个非拥有引用（non-owning reference）
///
/// Rc是单线程环境使用的。如果需要在多线程环境中使用，可以使用 `Arc`
fn main() {
    {
        let foo = Rc::new("foo");
        // foo 离开域，计数变为0，对象被销毁
    }

    {
        let bar = Rc::new("bar");
        let second_bar = Rc::clone(&bar);
        // second_bar 离开域
        // bar 离开域
    }

    {
        let baz = Rc::new("baz");
        {
            let second_baz = Rc::clone(&baz);
            // second_baz 离开域
        }
        // baz 离开域
    }

    let kid_one = spawn_kid_with_new_ball();
    let kid_two = Kid {
        ball: Rc::clone(&kid_one.ball),
    };
    let kid_three = Kid {
        ball: Rc::clone(&kid_one.ball),
    };
    // ball 会存活到这里
}

fn spawn_kid_with_new_ball() -> Kid {
    let ball = Rc::new(Ball);
    Kid {
        ball: Rc::clone(&ball),
    }
    // 即使这里 ball 已经离开域，但是它的计数仍然不为0，这个对象依然存活
}
