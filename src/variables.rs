// 静态可变变量
static mut MAX_LENGTH: usize = 100;

fn add_to_static_mut_variable() {
    unsafe {MAX_LENGTH += 1;}
}

pub fn use_static_mut_variable() {
    add_to_static_mut_variable();
    add_to_static_mut_variable();
    unsafe {
        println!("use static mut variable: {}", MAX_LENGTH);
    }
    
}
