use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
};

use serde::{Deserialize, Serialize};

/// TOML是关于k-v对的
/// message = "Hello World" -> 最简单的TOML文件
/// message = ["Hello", "world"] -> 值可以是数组
/// 一组k-v叫做表（table）
/// [smileys]
/// happy = ":)"
/// sad = ":("
/// 特别小的表可以在行内表示
/// smileys = { happy = ":)", sad = ":(" }
/// 表可以通过名称和 . 进行分离
/// [servers]
/// [servers.production]
/// ip = "1"
/// [servers.beta]
/// ip = "2"
/// TOML的一个很棒的属性是你可以将任何key转换成一个表，如果你想指定一些额外信息
/// [dependencies]
/// rocket_contrib = 0.3.3
/// ===>
/// [dependencies]
/// [dependencies.rocket_contrib]
/// version = "0.3.3"
/// default-features = false
/// ⚠️TOML的table中空格没有意义，可以进行缩进
/// 使用 # some comment 添加注释
///
/// Cargo本身也是使用toml来解析TOML文件的
fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("preferences.toml")
        .expect("failed to create TOML file");

    let buf_writer = BufWriter::new(&file);
    write_toml(buf_writer).expect("Failed to write TOML");

    let mut buf_reader = BufReader::new(&file);
    buf_reader
        .seek(SeekFrom::Start(0))
        .expect("Failed to jump to the beginning of the TOML file");
    read_toml(buf_reader).expect("Failed to read TOML");
}

type SerializeResult<T> = Result<T, toml::ser::Error>;

fn write_toml<W>(mut writer: W) -> SerializeResult<()>
where
    W: Write,
{
    let preferences = Preferences {
        person: Person {
            name: "Jan Nils Ferner".to_string(),
            email: "jn_ferner@hotmail.de".to_string(),
        },
        language: Language {
            display: "en-GB".to_string(),
            autocorrect: Some(vec![
                "en-GB".to_string(),
                "en-US".to_string(),
                "de-CH".to_string(),
            ]),
        },
        privacy: Privacy {
            share_anonymous_statistics: false,
            public_name: true,
            public_email: true,
        },
    };

    let toml = toml::to_string(&preferences)?;
    writer
        .write_all(toml.as_bytes())
        .expect("Failed to write file");

    Ok(())
}

type DeserializeResult<T> = Result<T, toml::de::Error>;

fn read_toml<R>(mut reader: R) -> DeserializeResult<()>
where
    R: Read,
{
    let mut toml = String::new();
    reader
        .read_to_string(&mut toml)
        .expect("Failed to read TOML");
    let preferences: Preferences = toml::from_str(&toml)?;

    println!("Personal data:");
    let person = &preferences.person;
    println!(" Name: {}", person.name);
    println!(" Email: {}", person.email);

    println!("\nLanguage prefereces:");
    let language = &preferences.language;
    println!(" Display language: {}", language.display);
    println!(" Autocorrect priority: {:?}", language.autocorrect);

    println!("\nPrivacy settings:");
    let privacy = &preferences.privacy;
    println!(
        " Share anonymous usage statistics: {}",
        privacy.share_anonymous_statistics
    );
    println!(" Display name publically: {}", privacy.public_name);
    println!(" Display email publically: {}", privacy.public_email);

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Preferences {
    person: Person,
    language: Language,
    privacy: Privacy,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct Language {
    display: String,
    autocorrect: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct Privacy {
    share_anonymous_statistics: bool,
    public_name: bool,
    public_email: bool,
}
