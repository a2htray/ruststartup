pub fn formatter_examples() {
    println!("正常使用 {}, {}", "hello", "world");
    println!("位置参数 {1}, {0}", "hello", "world");
    println!("具名参数 {hello}, {world}", hello="hello", world="world");

    println!("小数点精度 {:.2}", 3.1415926);
    println!("带符号的小数点精度 {:+.2}", -3.1415926);
    println!("使用空格补齐 5 个字符，{:5}!", "h");

    println!("不同进制的输出 - 二进制，{:#b}", 77);
    println!("不同进制的输出 - 八进制，{:#o}", 77);
    println!("不同进制的输出 - 小写十六进制 1，{:#x}", 77);
    println!("不同进制的输出 - 大写十六进制 2，{:#X}", 77);
    println!("不同进制的输出 - 不带前缀小写十六进制 3，{:x}", 77);
    println!("不同进制的输出 - 不带前缀大写十六进制 4，{:X}", 77);

    println!("使用了转义 \" {{ }}");

    let a = 1;
    println!("显示指针地址，{:p}", &a);
    println!("捕获变量，{a}")
}
