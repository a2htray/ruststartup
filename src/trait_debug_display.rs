/*
 Debug 和 Display 特性
*/
use std::fmt;

#[derive(Debug)]
pub struct School {
    name: String,
    location: String,
}

pub fn derive_debug_trait() {
    let school = School {
        name: String::from("XX 市第一中学"),
        location: String::from("幸福路 101 号"),
    };
    println!("派生于 Debug 特性，{:?}", school);
    println!("=> name: {}, location: {}", school.name, school.location);
}

pub struct Person {
    name: String,
    age: u8,
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "自定义实现 Debug 特性，name = {}，age = {}",
            self.name, self.age,
        )
    }
}

/// {:?} 作用于实现了 fmt::Debug 特性的类型
pub fn custom_debug_trait() {
    let person = Person {
        name: String::from("a2htray"),
        age: 35,
    };
    println!("{:?}", person);
}

pub enum Gender {
    Female(String),
    Male(String),
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Female(gender) => write!(f, "Female - {}", gender),
            Self::Male(gender) => write!(f, "Male - {}", gender),
        }
    }
}

/// {} 作用于实现了 fmt::display 特性的类型
pub fn custom_display_trait() {
    let gender = Gender::Female(String::from("女"));
    println!("自定义 Display 特性，gender = {}", gender);
}
