use core::fmt;
use std::{
    error,
    fs::File,
    io::{self, BufReader, Read},
    num, result,
};

#[derive(Debug)]
enum AgeReaderError {
    Io(io::Error),
    Parse(num::ParseIntError),
    NegativeAge(),
}

type Result<T> = result::Result<T, AgeReaderError>;

impl error::Error for AgeReaderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Io(ref err) => Some(err),
            Self::Parse(ref err) => Some(err),
            Self::NegativeAge() => None,
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}

impl fmt::Display for AgeReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Io(ref err) => write!(f, "IO error: {}", err),
            Self::Parse(ref err) => write!(f, "Parse error: {}", err),
            Self::NegativeAge() => write!(f, "Logic error: Age cannot be negative"),
        }
    }
}

impl From<io::Error> for AgeReaderError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<num::ParseIntError> for AgeReaderError {
    fn from(value: num::ParseIntError) -> Self {
        Self::Parse(value)
    }
}

/// 本例中，我们从文件中读取一个表示年龄的数字，可能碰到的错误包括
/// * 无法读取文件
/// * 无法转换文件内容为数字
/// * 数字是负数
///
/// 通常错误类型（枚举）变体的名称是它们所代表的错误类型
///
/// > 如果一个模块中有很多不同类型的错误，使用 module::TypeError 来导出它们，如果只有一种类型，那么直接使用 Error
///
/// 给自定义的错误类型实现 Error 特质，主要实现 依赖的 Display 特质和 source 方法
///
/// 一个crate的Error通常会在自己的 error 模块中，以便更好地组织，然后直接导出，对应的 lib.rs 中的内容包括
/// ```rust
/// mod error;
/// pub use error::Error;
/// ```
fn main() {
    const FILENAME: &str = "age.txt";
    let result = read_age(FILENAME);
    match result {
        Ok(num) => println!("{} contains the age {}", FILENAME, num),
        Err(AgeReaderError::Io(err)) => eprintln!("Failed to open the file {}: {}", FILENAME, err),
        Err(AgeReaderError::Parse(err)) => eprintln!(
            "Failed to read the contents of {} as a number: {}",
            FILENAME, err
        ),
        Err(AgeReaderError::NegativeAge()) => eprintln!("The age in the file is negative"),
    }
}

fn read_age(filename: &str) -> Result<i32> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    let age: i32 = content.trim().parse()?;
    if age.is_positive() {
        Ok(age)
    } else {
        Err(AgeReaderError::NegativeAge())
    }
}
