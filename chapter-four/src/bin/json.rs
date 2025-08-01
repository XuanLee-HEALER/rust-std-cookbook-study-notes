use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
};

use serde::{Deserialize, Serialize};

/// JSON由两种结构组成
/// * 一组k-v对，由 {} 包围，叫做对象，可以作为值
/// * 一组值，由 [] 包围，叫做数组
///
/// 当你的数据要被工具自动读取时，使用JSON更好，当你的数据要被人读或者修改时TOML更好
/// JSON不允许注释
///
/// 在Serde框架之下，所有格式的序列化和反序列化逻辑都隐藏在相同的特质定义之下，我们可以不关心内部实现，直接使用相同的API
///
/// JSON没有对应枚举 enum 的概念，Serde允许你通过注解来处理这种结构
fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("pet_owner.json")
        .expect("failed to create JSON file");

    let buf_writer = BufWriter::new(&file);
    write_json(buf_writer).expect("Failed to write JSON");

    let mut buf_reader = BufReader::new(&file);
    buf_reader
        .seek(SeekFrom::Start(0))
        .expect("Failed to jump to the beginning of the JSON file");
    read_json(&mut buf_reader).expect("Failed to read JSON")
}

fn write_json<W>(mut writer: W) -> serde_json::Result<()>
where
    W: Write,
{
    let pet_owner = PetOwner {
        name: "John".to_string(),
        age: 23,
        pets: vec![
            Pet {
                name: "Waldo".to_string(),
                species: AllowedSpecies::Dog,
                age: Some(2),
                colour: None,
            },
            Pet {
                name: "Speedy".to_string(),
                species: AllowedSpecies::Turtle,
                age: Some(47),
                colour: Some("Green".to_string()),
            },
            Pet {
                name: "Meows".to_string(),
                species: AllowedSpecies::Cat,
                age: None,
                colour: Some("Orange".to_string()),
            },
        ],
    };

    let json = serde_json::to_string_pretty(&pet_owner)?;

    writer
        .write_all(json.as_bytes())
        .expect("Failed to write file");
    Ok(())
}

fn read_json<R>(mut reader: R) -> serde_json::Result<()>
where
    R: Read,
{
    let mut json = String::new();
    reader
        .read_to_string(&mut json)
        .expect("Failed to read JSON");

    let pet_owner: PetOwner = serde_json::from_str(&json)?;
    println!("Pet owner profile:");
    println!(" Name: {}", pet_owner.name);
    println!(" Age: {}", pet_owner.age);

    println!("\nPets:");
    for pet in pet_owner.pets {
        println!(" Name: {}", pet.name);
        println!(" Species: {:?}", pet.species);
        if let Some(age) = pet.age {
            println!(" Age: {}", age);
        }
        if let Some(colour) = pet.colour {
            println!(" Colour: {}", colour);
        }
        println!()
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct PetOwner {
    name: String,
    age: u8,
    pets: Vec<Pet>,
}

#[derive(Serialize, Deserialize)]
struct Pet {
    name: String,
    species: AllowedSpecies,
    // 对很多 JSON key来说，可选很正常
    age: Option<u8>,
    colour: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum AllowedSpecies {
    Dog,
    Turtle,
    Cat,
}
