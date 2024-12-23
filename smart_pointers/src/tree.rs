use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub data: usize,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree() {
        let leaf = Rc::new(Node {
            data: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        {
            let node = Rc::new(Node {
                data: 10,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&node);

            println!("{:?}", leaf.parent.borrow().upgrade());
            assert_eq!(leaf.parent.borrow().upgrade().is_some(), true);
        }
        assert_eq!(leaf.parent.borrow().upgrade().is_none(), true);
    }
}
