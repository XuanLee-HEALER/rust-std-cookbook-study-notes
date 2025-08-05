use std::{sync::mpsc::channel, thread};

use rand::Rng;

/// 我们不需要将 tx 包裹进 Arc，因为它本身支持任意克隆
/// send() 将数据发送到跨线程的接收者，如果接收者不可用就会返回错误，例如太早被清理
/// 由于管道的两端是静态类型，如果你发送的是i32，那么你的管道只能处理i32
/// recv() 会阻塞主线程，所以也不需要 join 那些sender线程
///
/// rx（receiver）可以被遍历，每次获取一个元素都会阻塞主线程，和在循环中调用 recv() 是等价的，区别是当发送端不可用时会自动停止循环
///
/// channel 不是 Sync，只能在管道中移动，但是不能在它们之间共享。如果你需要可以 Sync 的管道，可以使用 sync_channel，当未应答消息的缓冲区满的时候会阻塞。spmc crate可以提供一个相反的管道实现
fn main() {
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("sending: {}", i);
            tx.send(i).expect("Disconnected from receiver");
        });
    }
    for _ in 0..10 {
        let msg = rx.recv().expect("Disconnected from sender");
        println!("received: {}", msg)
    }

    let (tx, rx) = channel();
    const DISCONNECT: &str = "Goodbye!";
    thread::spawn(move || {
        let mut rng = rand::rng();
        loop {
            let msg = match rng.random_range(0..5) {
                0 => "Hi",
                1 => DISCONNECT,
                2 => "Howdy there, cowboy",
                3 => "How are you?",
                4 => "I'm good, thanks",
                _ => unreachable!(),
            };
            println!("Sending: {}", msg);
            tx.send(msg).expect("Disconnected from receiver");
            if msg == DISCONNECT {
                break;
            }
        }
    });

    for msg in rx {
        println!("received: {}", msg)
    }
}
