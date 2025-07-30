/// 告诉rust你在使用某个crate，现在已经不需要写了（1.88）
extern crate rand;

fn main() {
    // 下面这些分布方式都是均匀分布，范围中每个数字出现概率是平均的
    let random_num1 = rand::random::<i32>();
    println!("random_num1: {}", random_num1);
    let random_num2: i32 = rand::random();
    println!("random_num2: {}", random_num2);

    let random_char = rand::random::<char>();
    println!("random_char: {}", random_char);

    use rand::Rng;
    // 使用一个可重用的生成器
    // Rng提供了gen方法，实际上还是调用了random方法
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        println!("This message has a 50-50 chance of being printed");
    }
    let random_num3 = rng.gen_range(0..10);
    println!("random_num3: {}", random_num3);

    let random_float = rng.gen_range(0.0..1.0);
    println!("random_float: {}", random_float);

    // 可以在一个generator创建过程中指定它的分布方式

    // 如果需要一个特殊的分布，下面这种写法已过期
    // let mut chacha_rng = rand::ChaChaRng::new_unseeded();
    // let random_chacha_num = chacha_rng.gen::<i32>();
    // println!("random_chacha_num: {}", random_chacha_num);
}
