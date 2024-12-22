#[derive(Debug, PartialEq)]
pub enum ConsList {
    Cons(u32, Box<ConsList>),
    Nil,
}
pub struct MyBox<T: Debug>(T);

impl<T: Debug> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

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
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        let mut curr = &list;
        let mut val = 1;
        while let Cons(item, next) = curr {
            println!("{item:?}, {next:?}");
            assert_eq!(val, *item);
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