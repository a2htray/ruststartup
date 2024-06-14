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

pub fn use_closure_in_struct() {
    println!("在结构中使用闭包");

    struct User<T>
    where
        T: Fn(Self) -> String,
    {
        get_name: T,
        name: String,
    }

    impl<T> User<T>
    where
        T: Fn(Self) -> String,
    {
        fn new(get_name: T, name: String) -> User<T> {
            User { get_name, name }
        }
    }

    // let get_name = |o| o.name

    // User::new(get_name, String::from("hello"));
}
