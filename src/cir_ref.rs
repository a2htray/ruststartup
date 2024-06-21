use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

pub fn use_list() {
    let list1 = List::Nil;
    let list2 = List::Cons(3, RefCell::new(Rc::new(list1)));

    println!("{:?}", list2.tail());
}
