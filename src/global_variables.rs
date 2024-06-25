/// 常量：不可变变量，常用于静态配置
/// 1. 定义常量必须声明类型

pub const MAX_ID: usize = usize::MAX / 2;

pub static mut COUNTER: usize = 0;

/// Rust 要求必须使用 unsafe 语句块才能访问和修改 static 变量
pub fn add_counter() {
    println!("对 static mut 变量进行计数相加");

    unsafe {
        COUNTER += 1;
        COUNTER += 1;
        assert_eq!(COUNTER, 2);
    }
}
