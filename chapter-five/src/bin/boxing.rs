use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{BufReader, Read},
};

/// 递归类型（包含自己类型成员的类型）是不能直接实现的，因为编译器需要在编译期类型的大小
/// 每个 `Box` 有相同的大小，它们是指向堆上一些类型的指针
#[derive(Debug)]
struct Node<T> {
    _data: T,
    child_nodes: Option<(BoxedNode<T>, BoxedNode<T>)>,
}

type BoxedNode<T> = Box<Node<T>>;

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            _data: data,
            child_nodes: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.child_nodes.is_none()
    }

    fn add_child_nodes(&mut self, a: Node<T>, b: Node<T>) {
        assert!(
            self.is_leaf(),
            "Tried to add child_nodes to a node that is not a leaf"
        );
        self.child_nodes = Some((Box::new(a), Box::new(b)))
    }
}

/// Box 可以让我们实现经典的多态（polymorphism）
trait Animal: Debug {
    fn sound(&self) -> &'static str;
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn sound(&self) -> &'static str {
        "Woof!"
    }
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn sound(&self) -> &'static str {
        "Meow!"
    }
}

/// 返回 Box<Iterator> 和 Box<Error> 会浪费一些性能，因为它必须将一个对象移到堆上
/// 对于 Box<Error> 的情况，应该创建自己的错误类型，将所有可能返回的错误类型组合到一起
/// 对于 Box<Iterator> ，你可以分析编译器的输出，找到你的程序到底返回的是什么，但这只适用于小的迭代器（⚠️可能是过时内容，现在已经被 dyn 替换）
fn main() {
    let mut root = Node::new(12);
    root.add_child_nodes(Node::new(3), Node::new(-24));
    root.child_nodes
        .as_mut()
        .unwrap()
        .0
        .add_child_nodes(Node::new(0), Node::new(1803));
    println!("Out binary tree looks like this: {:?}", root);

    // 书里的写法是 Box<Animal> 现在必须加 dyn 关键字
    // 使用这种方式我们对Box内的数据类型做了类型擦除
    let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
    zoo.push(Box::new(Dog {}));
    zoo.push(Box::new(Cat {}));
    for animal in zoo {
        println!("{:?} says {}", animal, animal.sound())
    }

    for word in caps_words_iter("do you feel lucky, punk?") {
        println!("{}", word);
    }

    let num = read_file_as_number("number.txt").expect("Failed read the file as a number");
    println!("number.txt contains the number {}", num);

    let multiplier = create_multiplier(23);
    let result = multiplier(3);
    println!("23 * 3 = {}", result)
}

/// 通过同样的（多态，类型擦除）机制，我们可以返回一个实现了Iterator特质的类型
fn caps_words_iter<'a>(text: &'a str) -> Box<dyn Iterator<Item = String> + 'a> {
    Box::new(text.trim().split(' ').map(|word| word.to_uppercase()))
}

fn read_file_as_number(filename: &str) -> Result<i32, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    let number: i32 = content.parse()?;
    Ok(number)
}

/// 我们可以在运行时创建、组合修改函数，就像在函数式语言中做的那样
fn create_multiplier(a: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |b| a * b)
}
