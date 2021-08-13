use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&l./eaf)]),
    });

    /*
      leaf的parent指向branch
     */
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    /*
     borrow获得不可变引用，upgrade把Weak<T>转换为Rc<T>
     */
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}