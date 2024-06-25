use std::ops::Deref;

/// Deref 特性
pub fn normal_deref() {
    let a = 1;
    let b = &a;

    println!("使用 * 号正常解引用 b = {}", *b);
}

/// 定义自己的智能指针
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/// 解引用操作符重载
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn use_deref() {
    let a = MyBox::new(5);

    println!("*a + 1 = {}", *a + 1);
}

/// Drop 特性

/// Box<T>
/// 使用场景
/// 1. 特意的将数据分配在堆上
/// 3. 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
/// 4. 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型
pub fn use_box_integer() {
    let a = Box::new(3);

    let b = *a + 1;

    println!("显式解引用 {}", b);
}

/// 2. 数据较大时，又不想在转移所有权时进行数据拷贝
pub fn large_scale_datastructure() {
    let a = [0; 1000];
    let a2 = a; // 发生值的深拷贝
    println!("{}", a2.len());

    let b = Box::new([0; 1000]);
    let b2 = b; // 发生所有权转移
    println!("{}", b2.len());
}
