fn main() {
    // 我们只关心需要的选项，而其它的选项使用默认值来替代
    let normal_burger = BurgerBuilder::new().build();
    let cheese_burger = BurgerBuilder::new().cheese(true).salad(true).build();
    let veggie_biamac = BurgerBuilder::new().vegetarian(true).patty_count(2).build();

    if let Ok(normal_burger) = normal_burger {
        normal_burger.print();
    }
    if let Ok(cheese_burger) = cheese_burger {
        cheese_burger.print();
    }
    if let Ok(veggie_biamac) = veggie_biamac {
        veggie_biamac.print();
    }

    // 检查无效选项
    let invalid_burger = BurgerBuilder::new().vegetarian(true).bacon(true).build();
    if let Err(e) = invalid_burger {
        println!("Failed to print burger: {}", e)
    }

    // 只要不调用build，可以无限重用builder
    let cheese_burger_builder = BurgerBuilder::new().cheese(true);
    for i in 1..10 {
        let cheese_burger = cheese_burger_builder.build();
        if let Ok(cheese_burger) = cheese_burger {
            println!("cheese burger number {} is ready!", i);
            cheese_burger.print();
        }
    }
}

/// builder应该包含**所有你想配置的值**
struct BurgerBuilder {
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool,
}

impl BurgerBuilder {
    // 标准值
    fn new() -> Self {
        Self {
            patty_count: 1,
            vegetarian: false,
            cheese: false,
            bacon: false,
            salad: true,
        }
    }

    // 对每个可配置的值定义一个方法
    fn patty_count(mut self, val: i32) -> Self {
        self.patty_count = val;
        self
    }

    fn vegetarian(mut self, val: bool) -> Self {
        self.vegetarian = val;
        self
    }

    fn cheese(mut self, val: bool) -> Self {
        self.cheese = val;
        self
    }

    fn bacon(mut self, val: bool) -> Self {
        self.bacon = val;
        self
    }

    fn salad(mut self, val: bool) -> Self {
        self.salad = val;
        self
    }

    /// 构建对象，返回 Result 是因为存在不合理的选项使我们无法构建成功
    /// 如果你的结构体不会因为无效配置创建失败，那么直接返回该结构体即可
    fn build(&self) -> Result<Burger, String> {
        let burger = Burger {
            patty_count: self.patty_count,
            vegetarian: self.vegetarian,
            cheese: self.cheese,
            bacon: self.bacon,
            salad: self.salad,
        };
        // 检查无效配置
        if burger.vegetarian && burger.bacon {
            Err("Sorry, but we don't server vegetarian bacon yet".to_string())
        } else {
            Ok(burger)
        }
    }
}

struct Burger {
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool,
}

impl Burger {
    fn print(&self) {
        let pretty_patties = if self.patty_count == 1 {
            "patty"
        } else {
            "patties"
        };
        let pretty_bool = |val| if val { "" } else { "no " };
        let pretty_vegetarian = if self.vegetarian { "vegetarian " } else { "" };
        println!(
            "This is a {}burger with {} {}, {}cheese, {}bacon and {}salad",
            pretty_vegetarian,
            self.patty_count,
            pretty_patties,
            pretty_bool(self.cheese),
            pretty_bool(self.bacon),
            pretty_bool(self.salad)
        )
    }
}
