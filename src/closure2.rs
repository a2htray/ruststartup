pub fn test_fn() {}

pub fn test_fn_mut() {}

pub fn test_fn_once() {
    // 定义一个可接受 FnOnce 特性的闭包
    // 捕获变量的方式包括：
    // 1. &T
    // 2. &mut T
    // 3. T
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let name = "a2htray";
    let print = || {
        println!("{} working", name);
    };

    apply(print);

    let mut count = 1;
    let inc = || {
        count += 1;
    };
    apply(inc);
    println!("inc 调用后，count = {}", count);
}
