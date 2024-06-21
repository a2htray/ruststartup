use core::slice;
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

/// 裸指针
/// 两种形式：
/// 1. *const T：不可变
/// 2. *mut T：可变
/// 用途一：解引用裸指针
pub fn raw_pointer() {
    let num = 5;

    let p = &num as *const i32;

    println!("{:?}", p);

    // 对一个裸指针解引用需要在 unsafe 块下
    unsafe {
        println!("{}", *p);
    }
}

// 数组指针
pub fn vec_pointer() {
    let vs = vec![1, 2, 3];

    let mut p = &vs[0] as *const i32;
    println!("{:?}", p);

    unsafe {
        p = p.wrapping_add(1);
        println!("{:?}", p);
        println!("{}", *p);

        p = p.add(1);
        println!("{:?}", p);
        println!("{}", *p);
    }
}

/// 基于引用创建裸指针
/// 创建裸指针是安全行为
pub fn create_raw_pointer() {
    let num = 5;
    let p1 = &num as *const i32;
    let p2 = &num as *const i32;

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
}

/// 基于内存地址创建裸指针
pub fn create_raw_pointer_from_address() {
    let address = 0x12345_usize;
    let p = address as *const i32;
    println!("{:?}", p);

    // unsafe {
    //     println!("{}", *p);
    // }
}

pub fn use_raw_pointer_from_address() {
    fn get_location_of_str() -> (usize, usize) {
        let s = "hello world";
        (s.as_ptr() as usize, s.len())
    }

    fn print_str(pointer: usize, length: usize) -> &'static str {
        unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    let (pointer, length) = get_location_of_str();

    println!(
        "get_location_of_str & print_str: {}",
        print_str(pointer, length)
    );
}

/// 基于智能指针创建裸指针
pub fn create_raw_pointer_from_box() {
    let a: Box<i32> = Box::new(3);

    let b = &*a as *const i32;

    let c: *const i32 = &*a;

    println!("create raw pointer from box {:?}, {:?}", b, c)
}

pub fn test_split_at_mut() {
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (part1, part2) = split_at_mut(&mut v, 2);

    println!("part1: {:?}, part2: {:?}", part1, part2);
}

// extern "C" {
//     fn abc(input: i32) -> i32;
// }

// pub fn test_ffi_abc() {
//     unsafe {
//         println!("C abc: {}", abc(-2));
//     }
// }

/// 实现 unsafe 特性
unsafe trait Fly {
    fn use_raw_pointer(&self);
}

struct Foo {
    v: i32,
}

unsafe impl Fly for Foo {
    fn use_raw_pointer(&self) {
        let ptr = &self.v as *const i32;

        unsafe {
            println!("use_raw_pointer: {}", *ptr);
        }
    }
}

pub fn unsafe_trait_use_raw_pointer() {
    let foo = Foo { v: 5 };
    foo.use_raw_pointer();
}
