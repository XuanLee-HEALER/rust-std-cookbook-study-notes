use std::{fmt::Display, ops::MulAssign};

/// 对于转换最重要的特质是 `From` 实现它就是定义如果从另一个类型的值得到此类型
///
/// > 对于能处理 Vec<T> 的代码，同时为 &[T] 实现一份是一个好的实践，这样得到了通用性和性能，因为你可以直接处理切片而不需要先将切片拷贝一份成新的vector
///
/// AsRef中的 as_ref 和 into 方法类似，只是它获取的是引用而不是所有权
/// 我们只需要给Vec实现AsRef，而不需要为切片实现，因为 &Vec<T> 会自动 deref-coerce 为 &[T]
/// AsMut和AsRef类似，只是它处理的是可变引用，要小心处理这个特质
///
/// 引用整洁API -> https://deterministic.space/elegant-apis-in-rust.html
fn main() {
    // the following three are equivalent
    let hello_world = "Hello World!".to_string();
    let hello_world: String = "Hello World!".into();
    let hello_world = String::from("Hello World!");

    let hello_world_bytes: Vec<u8> = "Hello World!".into();
    let hello_world_bytes = Vec::<u8>::from("Hello World!");

    let vec = vec![1, 2, 3];
    let double_vec = DoubleVec::from(vec);
    println!("Creating a DoubleVec from a Vec: {:?}", double_vec);

    let vec = vec![1, 2, 3];
    let double_vec: DoubleVec<_> = vec.into();
    println!("Converting a Vec into a DoubleVec: {:?}", double_vec);

    print_elements(double_vec.as_ref());
    // 标准库为 Option<T> 实现了 From<T> 特质，对于任意类型 T 都可以直接转换成 Option<T>
    easy_public_func(Some(1337), Some(123), None);
    ergonomic_public_func(1337, 123, None);
}

fn print_elements<T>(slice: &[T])
where
    T: Display,
{
    for elem in slice {
        print!("{} ", elem)
    }
    println!()
}

fn easy_public_func(foo: Option<i32>, bar: Option<i32>, baz: Option<i32>) {
    println!(
        "easy_public_func = foo: {:?}, bar: {:?}, baz: {:?}",
        foo, bar, baz
    )
}

// 只有当有很多可选参数时值得这样做
fn ergonomic_public_func<Foo, Bar, Baz>(foo: Foo, bar: Bar, baz: Baz)
where
    Foo: Into<Option<i32>>,
    Bar: Into<Option<i32>>,
    Baz: Into<Option<i32>>,
{
    let foo: Option<i32> = foo.into();
    let bar: Option<i32> = bar.into();
    let baz: Option<i32> = baz.into();

    println!(
        "ergonomic_pub_func = foo: {:?}, bar: {:?}, baz: {:?}",
        foo, bar, baz
    )
}

#[derive(Debug)]
struct DoubleVec<T>(Vec<T>);

impl<T> From<Vec<T>> for DoubleVec<T>
where
    T: MulAssign<i32>,
{
    fn from(mut value: Vec<T>) -> Self {
        for ele in &mut value {
            *ele *= 2
        }
        Self(value)
    }
}

impl<'a, T> From<&'a [T]> for DoubleVec<T>
where
    T: MulAssign<i32> + Clone,
{
    fn from(value: &'a [T]) -> Self {
        // Vec<T: MulAssign<i32> 自动实现了 Into<DoubleVec<T>>
        value.to_vec().into()
    }
}

impl<T> AsRef<Vec<T>> for DoubleVec<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}
