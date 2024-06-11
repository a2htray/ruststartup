// 在方法中使用泛型
// 支持加法操作的类型需要实现 std::ops::Add 特性
pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 使用 where 关键字
pub fn add2<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

pub fn test_add() {
    println!("泛型的加法 add(1, 1) = {}", add(1, 1));
    println!("泛型的加法 add(1.2, 1.2) = {}", add(1.2, 1.2));

    println!("where 关键字 - 泛型的加法 add2(1, 1) = {}", add2(1, 1));
    println!(
        "where 关键字 - 泛型的加法 add2(1.2, 1.2) = {}",
        add2(1.2, 1.2)
    );
}

// 在结构体中使用泛型
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_types(self) {
        println!(
            "x type = {}, y type = {}",
            std::any::type_name_of_val(&self.x),
            std::any::type_name_of_val(&self.y)
        )
    }
}

// distance 方法只对 T 为 f32 的结构可用
impl Point<f32> {
    fn distance(self, other: &Point<f32>) -> f32 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }
}

pub fn test_generic_struct() {
    let point = Point { x: 1.2, y: 1.2 };
    point.get_types();

    let pointf32 = Point {
        x: 1.2_f32,
        y: 1.2_f32,
    };
    let pointf32_other = Point {
        x: 1.3_f32,
        y: 2.2_f32,
    };
    println!(
        "Point<f32>.distance = {}",
        pointf32.distance(&pointf32_other)
    );
}

// const 泛型
// 可用于定义数组的长度值
fn iter_arr<T, const N: usize>(arr: &[T; N])
where
    T: std::fmt::Display,
{
    for item in arr {
        println!("const 泛型：{}", item);
    }
}

pub fn test_const_generic() {
    let arr1 = [1, 2, 3, 4];
    let arr2 = [5, 6, 7];
    iter_arr(&arr1);
    iter_arr(&arr2);
}
