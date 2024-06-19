use std::any::type_name_of_val;
use std::convert::TryInto;

pub fn test_as() {
    let a: i32 = 4;
    let b: u32 = 100;

    // 使用 as 进行强制转换
    // 范围小的类型往范围大的类型转换
    if (b as i32) > a {
        println!("b > a");
    }
}

pub fn as_examples() {
    // f64 -> i8
    let a = 3.1 as i8;
    // i8 -> i32
    let b = 100_i8 as i32;
    // char -> u8
    let c = 'a' as u8;

    println!("{} {} {}", a, b, c);
}

pub fn mem_addr_to_pointer() {
    let mut values: [i32; 2] = [1, 2];

    // 取到数组的第一个元素的内存地址
    let p1: *mut i32 = values.as_mut_ptr();

    println!("{:?}", p1);

    // 将内存地址转型成整数
    let first_address = p1 as usize;

    println!("{:?}", first_address);

    // +4 的原因在于 i32 的底层结构为 4 个字节
    let second_address = first_address + 4;

    // 将整数转换成内存地址
    let p2 = second_address as *mut i32;

    unsafe {
        // * 取值
        // 当 p2 所在内存地址的值自增加 1
        *p2 += 1;
    }

    assert_eq!(values[1], 3);
}

pub fn passing_as() {
    let a = 'a' as i32 as f64;

    assert_eq!("f64", type_name_of_val(&a));
}

pub fn try_into() {
    let a = 16;
    let b: u8 = a.try_into().unwrap();

    assert_eq!(16_u8, b);
    assert_eq!("u8", type_name_of_val(&b));

    let c: i32 = 1024;
    let _: u8 = match c.try_into() {
        Ok(u) => u,
        Err(e) => {
            println!("{}", e);
            0
        }
    };
}

pub fn common_type_cast() {
    struct Foo {
        a: i32,
        b: i32,
    }

    struct Bar {
        a: f32,
        b: f32,
    }

    let foo = Foo { a: 12, b: 12 };
}
