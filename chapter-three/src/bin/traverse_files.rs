use walkdir::{DirEntry, WalkDir};

/// walkdir包含三个重要的类型
/// * **WalkDir** 一个builder，构建目录访问器（walker）
/// * **IntoIter** builder构建的迭代器
/// * **DirEntry** 表示一个目录或者文件
///
/// OsStr是Rust直接和OS交互时使用的字符串类型。因为一些OS允许文件名中出现无效UTF-8
/// 对于这种文件名，有两种选择
/// 1. 让Rust将其转换成UTF-8，将无效字符替换成 Unicode Replacement Character（调用 to_string_lossy）
/// 2. 自己处理错误（to_str，检查返回的Option）
///
/// 如果你的应用仅面向Unix和Linux，可以通过 metadata().unwrap().perssions() 调用 访问文件的权限，通过 .mode() 可以看到 st_mode位，通过set_mode() 来设置新的位
fn main() {
    println!("All file paths in this directory:");
    // 可以直接将WalkDir当作一个迭代器访问当前目录下的内容
    // 通过 follow_links(true) 可以去查找软链接的父目录，可能会导致死循环
    for entry in WalkDir::new(".") {
        if let Ok(entry) = entry {
            // entry.path()返回rust原生的Path结构体，可以用在之后的操作中
            println!("{}", entry.path().display())
        }
    }

    println!("All non-hidden file name in this directory:");
    WalkDir::new("./chapter_three")
        .into_iter()
        // 特殊的迭代器adapter，对于filter的优化，如果对于目录，这个predicate返回false，那么不会再进入这个目录遍历，如果想只运用于文件而不是目录，只能使用普通的 filter
        .filter_entry(|entry| !is_hidden(entry))
        .filter_map(Result::ok)
        .for_each(|entry| {
            let name = entry.file_name().to_string_lossy();
            println!("{}", name)
        });

    println!("Paths of all subdirectories in this directory:");
    WalkDir::new(".")
        .into_iter()
        .filter_entry(is_dir)
        .filter_map(Result::ok)
        .for_each(|entry| {
            let path = entry.path().display();
            println!("{}", path)
        });

    let are_any_readonly = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| has_file_name(e, "vector.rs"))
        .filter_map(|e| e.metadata().ok())
        .any(|e| e.permissions().readonly());
    println!(
        "Are any the files called 'vector.rs' readonly? {}",
        are_any_readonly
    );

    let total_size = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());
    println!("Size of current directory: {} bytes", total_size);
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

/// 检查文件名是否包含有效的unicode
fn has_file_name(entry: &DirEntry, name: &str) -> bool {
    match entry.file_name().to_str() {
        Some(entry_name) => entry_name == name,
        None => false,
    }
}
