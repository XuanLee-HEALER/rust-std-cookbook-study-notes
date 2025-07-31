use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, BufWriter, Lines, Read, Seek, SeekFrom, Write},
};

/// File::open()以只读模式打开文件，返回该文件的handle，这个handle实现了 Read 特质
/// 使用BufReader，通过收集读指令可以极大改善访问资源的性能
///
/// 对于大文件，直接按行读取是低效的行为，使用BufReader会一次读取文件的一大部分，然后按行分批返回
///
/// File::create() 如果文件不存在就创建，否则会清空（truncate）文件
///
/// 使用 OpenOptions 来创建自己的文件handle，使用了builder模式
///
/// 当追加和读同一个handle的时候，存储当前读位置的指针可能在追加时移动
///
/// seek移动指针一些字节，然后返回新位置。SeekFrom::Current(0) 表示我们要移动的距离离我们现在的位置0字节
///
/// 我们需要调用flush，因为BufWriter会在它被丢弃（drop）时才真正去写，因为我们要在那之前读取，所以要使用此方法强制写
/// 最后使用 SeekFrom::Start(pos) 回到文件开头
///
/// 我们可以打开一个文件handle，然后在函数中到处传，这样提高了性能，避免重复对文件加锁和解锁，但是会导致其它进程无法访问文件
fn main() {
    let path = "./foo.txt";
    println!("Writing some data to '{}'", path);
    write_file(path, "Hello World!\n").expect("Failed to write to file");

    let content = read_file(path).expect("Failed to read file");
    println!("The file'{}' contains:", path);
    println!("{}", content);

    // 覆盖写
    println!("Writing new data to '{}'", path);
    write_file(path, "New content\n").expect("Failed to write to file");
    let content = read_file(path).expect("Failed to read file");
    println!("The file '{}' now contains:", path);
    println!("{}", content);

    // 追加写
    println!("Appending data to '{}'", path);
    append_file(path, "Some more content\n").expect("Failed to append the file");
    println!("The File '{}' now contains:", path);
    let lines = read_file_iterator(path).expect("Failed to read file");
    for line in lines {
        println!("{}", line.expect("Failed to read line"));
    }

    append_and_read(path, "Lest line in the file, goodbye").expect("Fail to read and write file")
}

fn read_file(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn read_file_iterator(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader.lines())
}

fn write_file(path: &str, content: &str) -> io::Result<()> {
    // 默认选项 create/write/truncate
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

fn append_file(path: &str, content: &str) -> io::Result<()> {
    let file = OpenOptions::new().append(true).open(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

fn append_and_read(path: &str, content: &str) -> io::Result<()> {
    let file = OpenOptions::new().read(true).append(true).open(path)?;
    let mut buf_reader = BufReader::new(&file);
    let mut buf_writer = BufWriter::new(&file);

    let mut file_content = String::new();
    buf_reader.read_to_string(&mut file_content)?;
    println!("File before appending:\n{}", file_content);

    let pos = buf_reader.seek(SeekFrom::Current(0))?;
    buf_writer.write_all(content.as_bytes())?;
    buf_writer.flush()?;
    buf_reader.seek(SeekFrom::Start(pos))?;

    buf_reader.read_to_string(&mut file_content)?;
    println!("File after appending:\n{}", file_content);

    Ok(())
}
