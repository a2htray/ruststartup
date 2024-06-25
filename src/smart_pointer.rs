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
#[derive(Debug)]
struct HasDrop1;
impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("drop HasDrop1");
    }
}

#[derive(Debug)]
struct HasDrop2;
impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("drop HasDrop2");
    }
}

#[derive(Debug)]
struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}
impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("drop HasTwoDrops")
    }
}

pub fn test_drops() {
    let has_two_drops = HasTwoDrops {
        one: HasDrop1 {},
        two: HasDrop2 {},
    };

    println!("{:?}", has_two_drops);
}

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

/// Rc<T>
/// 在堆上分配一个对象供程序的多个部分使用且无法确定哪个部分最后一个结束时，就可以使用 Rc 成为数据值的所有者
pub fn test_rc() {
    use std::rc::Rc;

    let a = Rc::new(String::from("hello world"));
    let b = Rc::clone(&a);

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));
}

pub fn test_rc2() {
    use std::rc::Rc;

    let a = Rc::new(String::from("hello world"));
    let b = Rc::clone(&a);

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(2, Rc::strong_count(&b));

    {
        let c = Rc::clone(&a);
        assert_eq!(3, Rc::strong_count(&c));
    }

    assert_eq!(2, Rc::strong_count(&b));
}
