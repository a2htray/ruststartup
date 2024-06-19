/*
Rust 中的错误主要分为两类：
1. 可恢复错误 -> Result<T, E>
2. 不可恢复错误 -> panic!
*/

use std::{
    any::Any,
    io::{self, Write},
};

// 触发 panic 的两种方式
// 1. 主动触发
pub fn panic_active_trigger() {
    // panic!("主动触发");
}

// 2. 被动触发
pub fn panic_passive_triger() {
    // let v = vec![1, 2, 3];

    // v[99]; // 发生越界
}

// 启用栈展开
// Windows  -> CMD           -> set RUST_BACKTRACE=1 && cargo run
//          -> Powershell    -> $env:RUST_BACKTRACE=1; cargo run
// Linux    -> RUST_BACKTRACE=1 cargo run

pub fn ip_parse() {
    use std::net::IpAddr;

    let local: IpAddr = "127.0.0.1".parse().unwrap();

    println!("{}", local.is_ipv4());
}

pub fn panic_macro() {
    panic!();
}

pub fn test_result_enum() {
    use std::fs::File;

    let f = File::open("hello_world.txt");
    let f = match f {
        Ok(f) => f,
        Err(_) => match File::create("hello_world.txt") {
            Ok(f) => f,
            Err(error) => {
                panic!("{}", error);
            }
        },
    };

    println!("{:?}", f.type_id());
}

pub fn error_propagation() {
    use std::fs::File;

    fn write_to_file(filename: &str) -> Result<usize, io::Error> {
        let mut f = match File::open(filename) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
        let s = "hello world";

        match writeln!(f, "{}", s) {
            Ok(_) => Ok(s.len()),
            Err(e) => return Err(e),
        }
    }

    match write_to_file("hello_world.txt") {
        Ok(n) => println!("size = {}", n),
        Err(e) => println!("{}", e),
    }
}
