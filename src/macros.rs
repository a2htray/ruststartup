/*
宏的分类：
1. 声明式宏
2. 过程宏
   1. 派生宏 #[derive]
   2. 类属性宏：为目标添加自定义的属性
   3. 类函数宏：类似于函数调用

宏可以拥有多个可变数量的参数
宏的替换发生在编译期，不会带来运行时的性能损耗
*/

#[macro_export]
macro_rules! do_something {
    ( $( $x:expr )+) => {
        {
            $(
                println!($x);
            )*
        }
    };
}

pub fn test_do_something_macro() {
    do_something!("123");
}
