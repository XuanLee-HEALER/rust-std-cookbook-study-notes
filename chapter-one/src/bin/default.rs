/// Rust中所有类型都有 Default 实现
/// 你自己的struct如果成员实现了 Default ，你可以使用 derive 来为你的struct自动实现 Default
/// 对于枚举或者复杂的结构体，需要自己实现 Default
///
/// 对于省略使用默认值，可以跳过任意多的值
fn main() {
    // 几乎所有基本类型都有默认值
    let foo: i32 = Default::default();
    println!("foo: {}", foo);

    let pizza: PizzaConfig = Default::default();
    println!("wants_cheese: {}", pizza.wants_cheese);

    println!("number_of_olives: {}", pizza.number_of_olives);

    println!("special message: {}", pizza.special_message);

    let crust_type = match pizza.crust_type {
        CrustType::Thin => "Nice and thin",
        CrustType::Thick => "Extra thick and extra filling",
    };
    println!("crust_type: {}", crust_type);

    let custom_pizza = PizzaConfig {
        number_of_olives: 12,
        ..Default::default()
    };

    let deluxe_custom_pizza = PizzaConfig {
        number_of_olives: 12,
        wants_cheese: true,
        special_message: "Will you marry me?".to_string(),
        ..Default::default()
    };
}

#[derive(Default)]
struct PizzaConfig {
    wants_cheese: bool,
    number_of_olives: i32,
    special_message: String,
    crust_type: CrustType,
}

enum CrustType {
    Thin,
    Thick,
}

impl Default for CrustType {
    fn default() -> Self {
        CrustType::Thin
    }
}
