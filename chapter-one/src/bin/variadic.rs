macro_rules! multiply {
    // 每个递归定义的开头是 edge case，这里的参数会让递归停止
    ($last:expr) => {
        $last
    };

    ($head:expr, $($tail:expr),+)=> {
        $head * multiply!($($tail),+)
    };
}

fn main() {
    let val = multiply!(2, 4, 8);
    println!("2*4*8 = {}", val)
}
