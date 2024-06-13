/*
 HashMap 的使用
 */

use std::collections::HashMap;

fn create_hashmap() -> HashMap<i32, String> {
    let mut hashmap = HashMap::new();

    hashmap.insert(1, String::from("red"));
    hashmap.insert(2, String::from("yellow"));
    hashmap.insert(3, String::from("green"));

    hashmap
}

pub fn create_hashmap_example() {
    let hashmap = create_hashmap();
    println!("create hashmap: {:?}", hashmap);
}

pub fn query_hashmap_example() {
    let hashmap = create_hashmap();
    let key = 1;
    let res = hashmap.get(&key);

    match res {
        Some(v) => println!("query hashmap value = {}", v),
        None => println!("not found"),
    }
}