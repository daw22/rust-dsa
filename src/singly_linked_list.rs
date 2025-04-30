use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

pub fn run_tests() {
    let mut list = List::new();
    list.append_front(1);
    list.append_last(2);
    list.append_front(0);
    list.append_last(3);
    assert_eq!(list.size, 4);
    list.print_list();
}

pub struct List<T: Display> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

pub struct Node<T: Display> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Display> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { data, next: None }))
    }
}

impl<T: Display> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn print_list(&self) {
        if let Some(head_node) = &self.head {
            let mut current_node = head_node.clone();
            println!("{}", current_node.borrow().data);
            while current_node.borrow().next.is_some() {
                let next_node = current_node.borrow().next.as_ref().unwrap().clone();
                println!("{}", next_node.borrow().data);
                current_node = next_node.clone();
            }
        } else {
            println!("list is empty");
        }
    }
    fn append_front(&mut self, data: T) {
        let new_node = Node::new(data);
        if let Some(head_node) = &self.head {
            new_node.borrow_mut().next = Some(head_node.clone());
            self.head = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        }
        self.size += 1;
    }

    fn append_last(&mut self, data: T) {
        let new_node = Node::new(data);
        if let Some(tail_node) = &self.tail {
            tail_node.borrow_mut().next = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        }
        self.size += 1;
    }
}
