trait EmailSender {
    fn send_mail(&self, msg: &Email) -> Option<String>;
}

#[derive(Debug, Clone)]
struct Email {
    from: String,
    to: String,
    msg: String,
}

#[derive(Debug)]
struct Customer {
    address: String,
    wants_news: bool,
}

fn publish_news(msg: &str, sender: &impl EmailSender, customers: &[Customer]) -> Option<i32> {
    let mut count = 0;
    let mut mail = Email {
        from: "Rust Newsletter".to_string(),
        to: "".to_string(),
        msg: msg.to_string(),
    };
    for customer in customers {
        if !customer.wants_news {
            continue;
        }
        mail.to = customer.address.to_string();
        if sender.send_mail(&mail).is_none() {
            return None;
        }
        count += 1
    }
    Some(count)
}

/// `RefCell` 是对于其他类型的包裹器，将借用检查器的规则检查从编译期移动到运行时，使用方式很简单
/// * borrow 借用底层值的不可变引用
/// * borrow_mut 借用底层值的可变引用
///
/// 这个结构最有用的就是本章展示的mock例子，为了测试目的而伪造对象
///
/// 还有很多类型使用了内部可变性，其中包括 Cell，它并不使用值的引用，而是拷贝（copy）它存储的值，对于保存基本类型很适合，因为这些类型都实现了 Copy
/// 其它包括 RwLock 和 Mutex，对于并行很重要
fn main() {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockEmailSender {
        sent_mails: RefCell<Vec<Email>>,
    }

    impl MockEmailSender {
        fn new() -> Self {
            MockEmailSender {
                sent_mails: RefCell::new(Vec::new()),
            }
        }
    }

    impl EmailSender for MockEmailSender {
        fn send_mail(&self, msg: &Email) -> Option<String> {
            self.sent_mails.borrow_mut().push(msg.clone());
            Some("200 OK".to_string())
        }
    }

    #[test]
    fn sends_zero_to_zero_customers() {
        let sent = publish_news("hello world!", &MockEmailSender::new(), &[]);
        assert_eq!(Some(0), sent)
    }

    #[test]
    fn sends_one_to_one_willing() {
        let customer = Customer {
            address: "herbert@herbert.com".to_string(),
            wants_news: true,
        };
        let sent = publish_news("Hello world!", &MockEmailSender::new(), &[customer]);
        assert_eq!(Some(1), sent)
    }

    #[test]
    fn sends_none_to_unwilling() {
        let customer_one = Customer {
            address: "herbert@herbert.com".to_string(),
            wants_news: false,
        };
        let customer_two = Customer {
            address: "michael@jackson.com".to_string(),
            wants_news: false,
        };
        let sent = publish_news(
            "hello world!",
            &MockEmailSender::new(),
            &[customer_one, customer_two],
        );
        assert_eq!(Some(0), sent)
    }

    #[test]
    fn sends_correct_mail() {
        let customer = Customer {
            address: "herbert@herbert.com".to_string(),
            wants_news: true,
        };
        let sender = MockEmailSender::new();
        publish_news("hello world!", &sender, &[customer]).expect("Failed to send mail");

        let mails = sender.sent_mails.borrow();
        assert_eq!(1, mails.len());
        assert_eq!("Rust Newsletter", mails[0].from);
        assert_eq!("herbert@herbert.com", mails[0].to);
        assert_eq!("hello world!", mails[0].msg)
    }
}
