#[derive(Debug, PartialEq)]
pub enum ConsList {
    Cons(u32, Box<ConsList>),
    Nil,
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
}
