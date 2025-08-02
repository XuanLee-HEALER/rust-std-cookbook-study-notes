use bitflags::{Flags, bitflags};

bitflags! {
    #[derive(Debug, Clone, Copy)]
    struct Spices:u32 {
        const SALT = 0b0000_0001;
        const PEPPER = 0b0000_0010;
        const CHILI = 0b0000_0100;
        const SAFFRON = 0b0000_1000;
        const ALL = Self::SALT.bits()|Self::PEPPER.bits()|Self::CHILI.bits()|Self::SAFFRON.bits() ;

    }
}

// 新版本的crate已经实现了clear方法，不需要再手动实现
// impl Spices {
//     pub fn clear(&mut self) -> &mut Self {
//         self.bits = 0;
//         self
//     }
// }

/// bitflags! 可以定义所有flags和它们的底层类型（本例中是u32），使用ALL_CAPS因为它们都是常量
/// 这个宏会创建一个结构体，有这些成员，并且实现了很多特质，支持 | & - ! 等操作和漂亮的打印机制（需要 derive 宏）
///
/// bit fields支持集合的操作，和HashSet类似
///
/// insert 激活一个位
/// remove 关闭一个位
/// intersects 检查两个位字段是否有任何一个位匹配
fn main() {
    let classic = Spices::SALT | Spices::PEPPER;
    let spicy = Spices::PEPPER | Spices::CHILI;
    println!("Classic: {:?}", classic);
    println!("Bits: {:08b}", classic.bits());
    println!("Spicy: {:?}", spicy);
    println!("Bits: {:08b}", spicy.bits());

    println!();

    println!("Union: {:?}", classic | spicy);
    println!("Intersection: {:?}", classic & spicy);
    println!("Difference: {:?}", classic - spicy);
    println!("Complement: {:?}", !classic);

    let mut custom = classic | spicy;
    println!("Custom spice mix: {:?}", custom);
    custom.insert(Spices::SAFFRON);
    println!("Custom spice after adding saffron: {:?}", custom);
    custom.toggle(Spices::CHILI);
    println!("Custom spice after toggling chili: {:?}", custom);
    custom.remove(Spices::SALT);
    println!("Custom spice after removing salt: {:?}", custom);

    let wants_salt = true;
    custom.set(Spices::SALT, wants_salt);
    if custom.contains(Spices::SALT) {
        println!("I hope I didn't put too much salt in it")
    }

    let bits = 0b0000_1101;
    if let Some(from_bits) = Spices::from_bits(bits) {
        println!("The bits {:08b} represent the flags {:?}", bits, from_bits)
    }

    custom.clear();
    println!("Custom spice mix after clearing: {:?}", custom)
}
