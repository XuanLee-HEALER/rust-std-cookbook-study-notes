use std::time::Instant;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// rayon 为所有实现了标准库的 Iterator 特质的类型实现了 `ParallelIterator` 特质。这个特质提供的功能和普通迭代器几乎相同，所有所有那些比较慢的普通迭代器都可以替换成 par_iter，对于移动的（获取所有权）的迭代器也有对应的 into_par_iter
/// rayon会将所有的工作平等的分配给可用的核心，但是要注意这里的并行没有顺序关系
fn main() {
    let legend = "Did you ever hear the tragedy of Darth Plagueis The Wise?";
    let words: Vec<_> = legend.split_whitespace().collect();

    words.par_iter().for_each(|val| println!("{}", val));

    let beg = Instant::now();
    let common_word_with_a: Vec<_> = words.iter().filter(|val| val.find('a').is_some()).collect();

    println!(
        "elapsed: {}ns, The following words contain the letter 'a' with common iterator: {:?}",
        beg.elapsed().as_nanos(),
        common_word_with_a
    );

    let beg = Instant::now();
    // par_iter可以做任何普通iterator能做的事情，以并行的方式
    let word_with_a: Vec<_> = words
        .par_iter()
        .filter(|val| val.find('a').is_some())
        .collect();

    println!(
        "elapsed: {}ns, The following words contain the letter 'a': {:?}",
        beg.elapsed().as_nanos(),
        word_with_a
    )
}
