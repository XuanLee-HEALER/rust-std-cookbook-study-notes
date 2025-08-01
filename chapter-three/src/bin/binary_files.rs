use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
};

use byteorder::{BE, ByteOrder, LE, ReadBytesExt, WriteBytesExt};

const PROTOCOL_START: &[u8] = b"MyProtocol";
const LITTLE_ENDIAN: &[u8] = b"LE";
const BIG_ENDIAN: &[u8] = b"BE";

/// 我们创建了一个个性化的二进制协议。它以一个 *magic number* 开始，即一个硬编码的值
/// 例如 .zip 的 魔数是 0x50 和 0x4B，表示ASCII的 PH，是发明人的名字缩写 PhilKatz
/// PDF，以 0x25 0x50 0x44 0x46，表示 PDF%
/// 之后的内容是LE或者BE表示这是大端序和小端序保存的
/// 之后的内容是负载，包含任意数量的u32数字
///
/// 应该将所有硬编码的内容都保存到常量中
fn main() {
    let path = "./bar.bin";
    write_dummy_protocol(path).expect("Failed write file");
    let payload = read_protocol(path).expect("Failed to read file");
    print!("The protocol contained the following payload: ");
    for num in payload {
        print!("0x{:X} ", num);
    }
    println!()
}

fn write_dummy_protocol(path: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);

    let magic = PROTOCOL_START;
    buf_writer.write_all(magic)?;

    let endianness = LITTLE_ENDIAN;
    buf_writer.write_all(endianness)?;

    buf_writer.write_u32::<LE>(0xDEAD)?;
    buf_writer.write_u32::<LE>(0xBEEF)?;

    Ok(())
}

fn read_protocol(path: &str) -> io::Result<Vec<u32>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);

    let mut start = [0_u8; 10];
    buf_reader.read_exact(&mut start)?;
    if &start != b"MyProtocol" {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Protocol didn't start with the expected magic string",
        ));
    }

    let mut endian = [0_u8; 2];
    buf_reader.read_exact(&mut endian)?;
    match &endian[..] {
        LITTLE_ENDIAN => read_protocol_payload::<LE, _>(&mut buf_reader),
        BIG_ENDIAN => read_protocol_payload::<BE, _>(&mut buf_reader),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to parse endianness",
        )),
    }
}

fn read_protocol_payload<E, R>(reader: &mut R) -> io::Result<Vec<u32>>
where
    E: ByteOrder,
    R: ReadBytesExt,
{
    let mut payload = Vec::new();
    const SIZE_OF_U32: usize = 4;
    loop {
        let mut raw_payload = [0; SIZE_OF_U32];
        match reader.read(&mut raw_payload)? {
            0 => return Ok(payload),
            SIZE_OF_U32 => {
                // 之所以要用 as_ref，是因为数组没有实现 Read
                let as_u32 = raw_payload.as_ref().read_u32::<E>()?;
                payload.push(as_u32);
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Payload ended unexpectedly",
                ));
            }
        }
    }
}
