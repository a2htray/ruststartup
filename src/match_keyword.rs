pub fn simple_match() {
    let v = 1;

    match v {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("others"),
    }

    let v_str = match v {
        1 => "one",
        2 => "two",
        _ => "others",
    };
    println!("{}", v_str);
}

pub fn use_or_operation_in_match() {
    let v = "a";

    match v {
        "a" | "b" => println!("a or b"),
        _ => println!("others"),
    }
}

/// match 操作中的解构
pub fn binding_in_match() {
    enum Color {
        Yellow(f32),
        Red(f32),
        Blue(f32),
    }

    let color = Color::Yellow(0.5);

    match color {
        Color::Yellow(opacity) => println!("Yellow: opacity-{}", opacity),
        Color::Red(opacity) => println!("Red: opacity-{}", opacity),
        Color::Blue(opacity) => println!("Blue: opacity-{}", opacity),
    }

    let colors = [Color::Yellow(0.2), Color::Red(0.3), Color::Blue(0.8)];

    for color in colors.iter() {
        match color {
            Color::Yellow(opacity) => println!("Yellow: opacity-{}", opacity),
            Color::Red(opacity) => println!("Red: opacity-{}", opacity),
            Color::Blue(opacity) => println!("Blue: opacity-{}", opacity),
        }
    }
}

/// matches! 宏：将表达式与模式进行匹配，返回 true 或 false
pub fn matches_macro() {
    let a = "a";
    println!("{}", matches!(a, "a"));
}
