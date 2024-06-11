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
