/// 定义特性
trait Run {
    fn run(&self);
}

struct Dog {
    name: String,
    color: String,
}

impl Run for Dog {
    fn run(&self) {
        println!("狗 `{}` 颜色 {} 在跑", self.name, self.color);
    }
}

struct Cat {
    name: String,
}

impl Run for Cat {
    fn run(&self) {
        println!("猫 `{}` 在跑", self.name);
    }
}

struct Zoo {
    // 使用 Box<dyn Run> 定义特性对象
    // 特性对象的内存大小不固定
    runnable_objs: Vec<Box<dyn Run>>,
}

impl Zoo {
    fn add_runnable_obj(&mut self, obj: Box<dyn Run>) {
        self.runnable_objs.push(obj);
    }
}

pub fn test_zoo() {
    let mut zoo = Zoo {
        runnable_objs: vec![
            Box::new(Cat {
                name: String::from("虎猫"),
            }),
            Box::new(Dog {
                name: String::from("陆虎"),
                color: String::from("黄色"),
            }),
        ],
    };

    zoo.add_runnable_obj(Box::new(Cat {
        name: String::from("小花猫"),
    }));

    for obj in zoo.runnable_objs {
        obj.run();
    }
}

trait Swin {
    // 使用 Self 指代特性
    fn swin(&self) -> &Self;
}

impl Swin for Dog {
    // 使用 Self 指代结构
    fn swin(&self) -> &Self {
        println!("狗 {} 在游泳", self.name);
        self
    }
}

pub fn test_dog_swin_and_run() {
    let dog = Dog {
        name: String::from("陆虎"),
        color: String::from("黄色"),
    };

    dog.swin().run();
}

// 特性对象的限制
// 特性方法的返回类型不能是 Self
// 特性方法不能含有泛型参数
