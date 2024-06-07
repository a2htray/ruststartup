/*
 定义枚举类型的 3 种不同格式
 * 不带值
 * 带值
 * 带不同类型的值
*/

/// 汽车级别，A 级车、B 级车、C 级车和 D 级车
pub enum CarLevel {
    A,
    B,
    C,
    D,
}

pub fn print_car_level() {
    let car_level = CarLevel::A;

    let car_level_str = match car_level {
        CarLevel::A => "A 级车",
        CarLevel::B => "B 级车",
        CarLevel::C => "C 级车",
        CarLevel::D => "D 级车",
    };

    println!("枚举类型的汽车级别：{}", car_level_str);
}

pub enum CarLevelWithBrand {
    A(String),
    B(String),
    C(String),
    D(String),
}

pub fn print_car_level_with_brand() {
    let car_level = CarLevelWithBrand::B(String::from("Honda"));

    match car_level {
        CarLevelWithBrand::A(brand) => println!("带值的枚举类型 A 级车：{}", brand),
        CarLevelWithBrand::B(brand) => println!("带值的枚举类型 B 级车：{}", brand),
        CarLevelWithBrand::C(brand) => println!("带值的枚举类型 C 级车：{}", brand),
        CarLevelWithBrand::D(brand) => println!("带值的枚举类型 D 级车：{}", brand),
    }
}

pub enum CarLevelWithBrandOrPrice {
    A(String, f32),
    B { brand: String, price: f32 },
    C(String),
    D,
}

pub fn print_car_level_with_brand_or_price() {
    let car_level = CarLevelWithBrandOrPrice::B {
        brand: String::from("Honda"),
        price: 21.00,
    };

    match car_level {
        CarLevelWithBrandOrPrice::A(brand, price) => println!(
            "不同值类型的枚举类型 A 级车：品牌-{}，价格-{}",
            brand, price
        ),
        CarLevelWithBrandOrPrice::B { brand, price } => println!(
            "不同值类型的枚举类型 B 级车：品牌-{}，价格-{}",
            brand, price
        ),
        CarLevelWithBrandOrPrice::C(brand) => {
            println!("不同值类型的枚举类型 C 级车：品牌-{}，价格-无", brand)
        }
        CarLevelWithBrandOrPrice::D => println!("不同值类型的枚举类型 D 级车：品牌-无，价格-无"),
    }
}
