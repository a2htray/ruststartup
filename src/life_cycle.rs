pub fn trigger_dangling_pointer() {
    #![allow(warnings)]
    let x;
    {
        let y = 5;
        x = &y;
    } // 离开该作用域，y 被释放；

    // 此时 x 引用了一个无效的变量

    println!("触发悬垂指针 ...");
}

// 'a 声明生命周期
// 返回值与参数 xs1 和 xs2 具有相同的生命周期
pub fn test_which_bigger_vector<'a>(xs1: &'a Vec<f32>, xs2: &'a Vec<f32>) -> &'a Vec<f32> {
    if xs1.len() > xs2.len() {
        xs1
    } else {
        xs2
    }
}

/// 返回两中字符串中较长的字符串
/// 生命周期标注并不会改变任何引用的实际作用域
/// <'a>：声明一个生命周期，表示大于等于生命周期 a
/// &'a str：表示传参 x、传参 y 和返回值具有相同的生命周期 a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn test_longest() {
    let (x, y) = ("1234", "12345");
    println!("longest {}", longest(x, y));
}

pub fn test_longest2() {
    let x = "1234";
    {
        let y = "12345";
        println!("longest {}", longest(x, y));
    }
}

/// 三条生命周期消除规则
/// 1. 每一个引用参数都会获得独自的生命周期
///
/// ```
/// fn rule1(x: &str, y: &str) {}
/// ```
/// <==>
/// ```
/// fn rule1<'a, 'b>(x: &'a str, y: &'b str) {}
/// ```
///
/// 2. 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期
///
/// ```
/// fn rule2(x: &str) -> &str {}
/// ```
/// <==>
/// ```
/// fn rule2<'a>(x: &'a str) -> &'a str {}
/// ```
///
/// 3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期
pub fn three_rules() {
    println!("三条生命周期消除规则");
}

/*
在结构中使用生命周期
*/

struct Foo<'a> {
    bar: &'a str,
}

impl<'a> Foo<'a> {
    /// self 和返回值具有相同的生命周期
    fn baz(&'a self) -> &str {
        return self.bar;
    }
}

pub fn use_life_circle_in_struct() {
    let foo = Foo { bar: "bar" };
    println!("foo.bar = {}", foo.baz());
}
