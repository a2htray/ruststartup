/*
 数组的使用
*/

fn create_vec1() -> Vec<i32> {
    // 使用 Vec::new() 创建数组
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    vec
}

fn create_vec2() -> Vec<i32> {
    // 使用 vec! 创建数组
    let vec = vec![4, 5, 6];

    vec
}

fn add_new_elements(vec: &mut Vec<i32>) {
    vec.push(1);
    vec.push(2);
    vec.push(3);
}

fn remove_front_3_elements(vec: &mut Vec<i32>) {
    let mut n = 3;
    if vec.len() < n {
        n = vec.len();
    }

    for i in 0..=n {
        vec.remove(i);
    }
}

fn iter_vec1() {
    let vec = vec![1, 2, 3];
    let mut i = 0;

    // 遍历值
    for ele in vec {
        println!("遍历数组 {}-{}", i, ele);
        i += 1;
    }
}

fn iter_vec2() {
    let vec = vec![1, 2, 3];
    let mut i = 0;

    // 遍历值
    for ele in vec.iter() {
        println!("遍历数组 {}-{}", i, ele);
        i += 1;
    }
}

fn iter_vec3() {
    let vec = vec![1, 2, 3];

    // 遍历值和索引
    for (i, ele) in vec.iter().enumerate() {
        println!("遍历数组 {}-{}", i, ele);
    }
}

pub fn create_vec() {
    let vec1 = create_vec1();
    println!("result vector = {:?}", vec1);

    let vec2 = create_vec2();
    println!("result vector = {:?}", vec2);
}

pub fn add_and_remove_elements() {
    let mut vec = vec![1, 2, 3, 4];
    add_new_elements(&mut vec);
    println!("after adding elements, result vector = {:?}", vec);

    remove_front_3_elements(&mut vec);
    println!("after removing elements, result vector = {:?}", vec);
}

pub fn iter_vec() {
    iter_vec1();
    iter_vec2();
    iter_vec3();
}
