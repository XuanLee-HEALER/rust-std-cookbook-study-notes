use std::borrow::Cow;

fn main() {
    let name_length = NameLength::new_("John");
    name_length.print();
}

struct NameLength<'a> {
    name: Cow<'a, str>,
    length: usize,
}

impl<'a> NameLength<'a> {
    /// 如果有这个方法 new -> Self，这个结构体的用户不会配置或者依赖这个结构体的成员，因为它们被认为是内部状态
    /// 习惯：如果有这个函数，那就用这个函数来初始化结构体
    /// 这个模式可以用来隐藏实现
    ///
    /// 即使有其它的方法可以初始化结构体，最好也提供一个 new 函数作为默认的初始化方式，例如 Vec::new 和 Vec::with_capacity
    ///
    /// Cow在Rust中是对一个类型的 Clone On Write 包裹体，即只要允许就借用类型，必要时才会创建自有类型（第一次被修改）
    // fn new(name: &str) -> Self {
    //     NameLength {
    //         name: name.to_string(),
    //         length: name.len(),
    //     }
    // }

    /// https://jwilm.io/blog/from-str-to-cow/
    ///
    fn new_<S>(name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let name: Cow<'_, str> = name.into();
        NameLength {
            // 赋值顺序有影响 ⚠️
            length: name.len(),
            name,
        }
    }

    fn print(&self) {
        println!(
            "The name '{}' is '{}' characters long",
            self.name, self.length
        )
    }
}
