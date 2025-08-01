use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, SeekFrom, prelude::*};
use std::{fs::File, io};

use flate2::{Compression, bufread::ZlibDecoder, bufread::ZlibEncoder};

/// ⚠️flate2版本的Encoder的使用方式已经是接收一个可读流，返回一个压缩后的可读流，不再需要手动调用finish了
fn main() {
    let bytes = b"I have a dream that one day this nation will rise up, \
    and live out the true meaning of its creed";
    println!("Original: {:?}", bytes.as_ref());

    let encoded = encode_bytes(bytes.as_ref()).expect("Failed to encode bytes");
    println!("Encoded: {:?}", encoded);

    let decoded = decode_bytes(&encoded).expect("Failed to decode bytes");
    println!("Decoded: {:?}", decoded);

    let original = File::open("ferris.png").expect("Failed to open file");
    let mut original_reader = BufReader::new(original);

    let data = encode_file(&mut original_reader).expect("Failed to encode file");

    let encoded = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("ferris_encoded.zlib")
        .expect("Failed to create encoded file");
    let mut encoded_reader = BufReader::new(&encoded);
    let mut encoded_writer = BufWriter::new(&encoded);

    encoded_writer
        .write_all(&data)
        .expect("Failed to write encoded file");

    encoded_reader
        .seek(SeekFrom::Start(0))
        .expect("Failed to reset file");

    let data = decode_file(&mut encoded_reader).expect("Failed to decode file");

    let mut decoded = File::create("ferris_decoded.png").expect("Failed to create decoded file");
    decoded
        .write_all(&data)
        .expect("Failed to write decoded file");
}

fn encode_bytes(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut result = vec![];
    ZlibEncoder::new(bytes, Compression::default()).read_to_end(&mut result)?;
    Ok(result)
}

fn decode_bytes(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut result = vec![];
    ZlibDecoder::new(bytes).read_to_end(&mut result)?;
    Ok(result)
}

// 压缩和解压缩（下）方法中将结果存放到字节数组（buffer）中浪费了一些性能
// 对于库函数，可以提供一个方法，让压缩/解压缩结果直接通过 io::copy 从结果流写入目标流
fn encode_file(file: &mut impl BufRead) -> io::Result<Vec<u8>> {
    let mut result = vec![];
    ZlibEncoder::new(file, Compression::best()).read_to_end(&mut result)?;
    Ok(result)
}

fn decode_file(file: &mut impl BufRead) -> io::Result<Vec<u8>> {
    let mut result = vec![];
    ZlibDecoder::new(file).read_to_end(&mut result)?;
    Ok(result)
}
