pub fn simple_closure() {
    println!("简单的闭包示例");

    let x = 1;

    // 利用编译器的类型推导能力
    let sum = |y| x + y;

    sum(2);

    // 显式标注类型
    let sum = |y: i32| -> i32 { x + y };

    sum(3);
}

pub fn capture_outer_variable() {
    let name = "a2htray";

    let working = || {
        println!("{} 正在工作", name);
    };

    working();
}

pub fn determine_closure_type() {
    let add = |a, b| a + b;
    println!("用 i32 作为参数类型 {}", add(1, 2));
    // println!("这里就不能用 f32 作为参数类型了，执行会报错 {}", add(1.2, 2.2));
}

pub fn rust_by_example1() {
    let outer_var = 42;

    // 在签名中声明参数类型
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    // 利用编译器的类型推导能力
    let closure_inferred = |i| i + outer_var;

    // 调用闭包
    println!("签名中声明类型: {}", closure_annotated(1));
    println!("编译器类型推导: {}", closure_inferred(1));

    // 没有输入参数、返回值类型为 i32 的闭包
    // 返回值类型由编译器推导
    let one = || 1;
    println!("调用闭包返回 1: {}", one());
}

pub fn rust_by_example_capturing() {
    use std::mem;

    let color = String::from("green");

    // 闭包会使用借用（&）的方式使用 color，该借用会持续到最后一次 print 的调用
    // println! 的输入参数类型是不可变引用
    let print = || println!("`color`: {}", color);

    // 调用闭包
    print();

    // 因为闭包只是借用，所以后面还可以对 color 变量进行借用
    let _reborrow = &color;
    print();

    // 触发移动
    let _color_moved = color;

    let mut count = 0;

    // count 变量自增，此处可以使用 `&mut count` 或 `count`
    // inc 前的 mut 是必须的，这样才会发生可变借用
    // 因此在闭中的 count 需要是一个 `&mut count`
    //
    // 如果你希望在闭包中修改一个值，则需要在闭包声明时加入 mut 关键字
    // 同时，捕获的变量也应该是 `&mut`
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 当前闭包依然持有 count 的可变借用，所以不能在此时进行借用
    // 再次调用闭包
    inc();

    // inc 闭包不需要在执行，也就不会持有 `&mut count`，因此下面的可变借用有效
    let _count_reborrowed = &mut count;

    // Box<i32> 不具备 Copy 特性
    let movable = Box::new(3);

    // `mem::drop` 需要一个 `T`，所以它使用一个值
    // 在调用 consume 时，会发生移动
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 闭包只能调用一次，因为 movable 变量已经发生了移动
    consume();
    // consume();
    // ^ TODO: 取消注释，再次调用 consume 会报错
}

pub fn rust_by_example_move() {
    // `Vec` 不具备 Copy 特性
    let haystack = vec![1, 2, 3];

    // move 强制发生所有权的转移
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // 在后续程序中不能再使用 haystack 变量
    // 如果移去 move 关键字，则在后续程序中可以再使用 haystack 变量
}

pub fn test_owner() {
    let s = "hello";
    let my_print = || {
        println!("测试闭包会不会导致所有权改变 {}", s);
    };

    my_print();
    println!("测试闭包会不会导致所有权改变 {}", s);
}

pub fn fn_variable_declare() {
    let add: Box<dyn Fn(i32, i32) -> i32> = Box::new(|a, b| a + b);
    println!(
        "将闭包函数类型对变量进行声明，需要使用 Box<dyn XXX> {}",
        add(1, 2)
    );
}

pub fn fn_as_parameter() {
    let add = |x: i32, y: i32| -> i32 { x + y };

    fn apply_fn(fn_callback: impl Fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
        fn_callback(x, y)
    }

    let v = apply_fn(add, 1, 2);

    println!("函数作为参数 {}", v);
}

pub fn fn_as_parameter2() {
    // 定义一个函数并将其作为参数传入另一个函数
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn use_add_func(add_func: impl Fn(i32, i32) -> i32, a: i32, b: i32) {
        let sum = add_func(a, b);
        println!("定义一个函数并将其作为参数传入另一个函数, {}", sum);
    }

    use_add_func(add, 1, 2);
}

pub fn fn_as_return() {
    fn create_fn(t: i32) -> impl Fn(i32, i32) -> i32 {
        match t {
            1 => |x, y| -> i32 { x + y },
            _ => |x: i32, y: i32| -> i32 { x - y },
        }
    }

    let fn_created = create_fn(1);

    println!("函数作为返回值 {}", fn_created(1, 2));
    println!("函数作为返回值 {}", create_fn(2)(1, 2));
}
