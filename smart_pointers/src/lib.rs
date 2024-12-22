use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug, PartialEq)]
pub enum ConsList {
    Cons(u32, Rc<ConsList>),
    Nil,
}

impl Clone for ConsList {
    fn clone(&self) -> Self {
        match self {
            Self::Nil => Self::Nil,
            Self::Cons(value, next) => Self::Cons(*value, Rc::clone(next)),
        }
    }
}

pub struct MyBox<T: Debug>(T);

impl<T: Debug> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Debug> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Debug> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Debug> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Droping my box: {:?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cons_list() {
        use ConsList::{Cons, Nil};
        // Use while let to go through entire list
        let list = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
        let mut curr = Rc::clone(&list);
        let mut val = 1;
        while let Cons(item, next) = (*curr).clone() {
            println!("{item:?}, {next:?}");
            assert_eq!(val, item);
            val += 1;
            curr = next;
        }
        assert_eq!(*curr, Nil);
    }

    #[test]
    fn my_box() {
        let val = 9;
        let my_box = MyBox::new(val);
        assert_eq!(val, 9);
        assert_eq!(*my_box, 9);

        fn hello(str: &str) {
            println!("{str}")
        }

        fn hello_mut(str: &mut str) {
            println!("{str}")
        }

        let my_str_box = MyBox::new(String::from("my_str_box"));
        hello(&my_str_box);
        let mut my_mut_str_box = MyBox::new(String::from("my_str_mut_box"));
        hello(&my_mut_str_box);
        hello_mut(&mut my_mut_str_box);
        drop(my_str_box);
    }
}
