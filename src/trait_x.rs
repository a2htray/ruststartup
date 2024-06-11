/*
 特性示例代码
*/

use std::fmt::{Debug, Display};

/// 自动派生特性
#[derive(Debug)]
struct Animal {
    name: String,
}

pub fn test_auto_derive_trait() {
    let animal = Animal {
        name: String::from("Jack"),
    };
    println!(
        "自动派生特性 Debug: animal {:?} with name = {}",
        animal, animal.name
    );
}

/// 定义特性
trait Fly {
    fn fly(&self);
}

struct Bird {
    name: String,
}

impl Fly for Bird {
    fn fly(&self) {
        println!("我是一只鸟 {}，肯定会飞", self.name);
    }
}

struct Fish {
    name: String,
}

impl Fly for Fish {
    fn fly(&self) {
        println!("虽然我是一条鱼 {}，但我会滑翔", self.name);
    }
}

pub fn test_custom_fly_trait() {
    let bird = Bird {
        name: String::from("Jack"),
    };
    let fish = Fish {
        name: String::from("Ann"),
    };
    bird.fly();
    fish.fly();
}

/// 特性作为参数
/// fly_obj 是实现了 Fly 特性的结构的实例
fn call_fly(fly_obj: &impl Fly) {
    fly_obj.fly();
}

/// call_fly(fly_obj: &impl Fly) 实际上是以下方法的语法糖
fn call_fly2<T: Fly>(fly_obj: &T) {
    fly_obj.fly();
}

pub fn test_trait_as_parameter() {
    let bird = Bird {
        name: String::from("Jack"),
    };
    let fish = Fish {
        name: String::from("Ann"),
    };

    call_fly(&bird);
    call_fly(&fish);

    call_fly2(&bird);
    call_fly2(&fish);
}

/// 多重约束
/// 使用 + 定义多重约束
fn multiple_trait_bounds<T: Debug + Display>(obj: &T) {
    println!("使用 + 定义多重约束 {}", obj);
    println!("使用 + 定义多重约束 {:?}", obj);
}

struct Point {
    x: f32,
    y: f32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Debug output => x = {}, y = {}", self.x, self.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Display output => x = {}, y = {}", self.x, self.y)
    }
}

pub fn test_multiple_trait_bounds() {
    let s = "123";
    multiple_trait_bounds(&s);

    let point = Point { x: 1.2, y: 1.2 };
    multiple_trait_bounds(&point);
}

/// where 约束
fn multiple_trait_bounds_where<T>(obj: &T)
where
    T: Debug + Display,
{
    println!("使用 + 和 where 定义多重约束 {}", obj);
    println!("使用 + 和 where 定义多重约束 {:?}", obj);
}

pub fn test_multiple_trait_bounds_where() {
    let s = "123";
    multiple_trait_bounds_where(&s);

    let point = Point { x: 1.2, y: 1.2 };
    multiple_trait_bounds_where(&point);
}
