use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Seek, SeekFrom};

/// *endianness* 序，这是描述buffer（内存）中的值是如何排序的方式，有两种排序方式
/// 可以将内存看作一个大数组，从头到尾每个地址（address）可以保存一个字节（Byte），4位，大端序和小端序的区别在于将任意一个字（word，32bit）存储到内存中，它的高位放到内存更靠前的位置还是低位更靠前的位置
/// * 从低位到高位排列（小端序），
/// * 从高位到低位（大端序）
///
/// 关于序的讲解 https://web.archive.org/web/20170808042522/http://www.cs.umd.edu/class/sum2003/cmsc311/Notes/Data/endian.html
/// 假设我们要保存一个十六进制数，0x90AB12CD 首先我们要把它分成一些位 0x90 0xAB 0x12 0xCD
/// 首先使用大端序排列：0x90 - 0xAB - 0x12 - 0xCD
/// 然后使用小端序排序：0xCD - 0x12 - 0xAB - 0x90
///
/// 微处理器，例如Intel使用小端序，互联网协议，例如TCP，IPv4、IPv6和UDP使用大端序
///
/// 首先我们需要一个二进制数据源，例子中使用了vector。然后我们将其包裹进Cursor，提供了对 Seek 的实现
/// 使用 {:b} 可以使用二进制打印数据
/// 要注意 SeekFrom::End(n) 只会基于最后一个位置往后数而不会往前，中间空出的位置只需要用0填补（写的时候）
/// 读写超过1个字节的数字需要指定 序 （通过类型注解）。要注意当你写超过结尾之后，总是会扩展buffer
///
/// 使用 NativeEndian 可以设置成你的操作系统的默认序， NetworkEndian 是大端序，还有BE、LE缩写，在crate中都有定义
fn main() {
    let binary_nums = vec![2, 3, 12, 8, 5, 0];
    // 将二进制集合放到一个cursor中，获取seek能力
    let mut buff = Cursor::new(binary_nums);
    let first_byte = buff.read_u8().expect("Failed to read byte");
    println!("first byte in binary: {:b}", first_byte);

    let second_byte_as_int = buff.read_i8().expect("Failed to read byte as int");
    println!("second byte as int: {}", second_byte_as_int);

    println!("Before: {:?}", buff);
    // 读写都会使位置加1
    buff.write_u8(123).expect("Failed to overwrite a byte");
    println!("After: {:?}", buff);

    buff.seek(SeekFrom::End(0)).expect("Failed to seek end");
    println!("Last position: {}", buff.position());

    // 087B0302 142279426
    buff.set_position(0);
    let as_u32 = buff
        .read_u32::<LittleEndian>()
        .expect("Failed to read bytes");
    println!(
        "First four bytes as u32 in little endian order:\t{}",
        as_u32
    );

    buff.set_position(0);
    // 02037B08 33782536
    let as_u32 = buff.read_u32::<BigEndian>().expect("Failed to read bytes");
    println!("First four bytes as u32 in big endian order:\t{}", as_u32);

    println!("Before appending: {:?}", buff);
    buff.seek(SeekFrom::End(0)).expect("Failed to seek end");
    buff.write_f32::<LittleEndian>(-33.4)
        .expect("Failed to write to end");
    println!("After appending: {:?}", buff);

    let mut read_buffer = [0; 5];
    buff.set_position(0);
    buff.read_u16_into::<LittleEndian>(&mut read_buffer)
        .expect("Failed to read all bytes");
    println!(
        "All bytes as u16s in little endian order: {:?}",
        read_buffer
    )
}
