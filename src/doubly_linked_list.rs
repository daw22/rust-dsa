use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::{Rc, Weak};

#[derive(Default)]
struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I am {}, {} years old.", self.name, self.age)
    }
}

pub fn run_tests() {
    let mut list: List<Person> = List::default();
    let me = Person {
        name: "dawit".to_string(),
        age: 30,
    };
    let abulu = Person {
        name: "abulu".to_string(),
        age: 20,
    };
    list.append_node_front(me);
    list.append_node_behind(abulu);
    //let head_node = list.head.as_ref().unwrap().clone();
    let tail_node = list.tail.as_ref().unwrap().clone();
    let kore = Person {
        name: "kore".to_string(),
        age: 10,
    };
    list.append_after(tail_node, kore);
    let dawa = Person {
        name: "dawa".to_string(),
        age: 100,
    };
    let a = Person {
        name: "abebe".to_string(),
        age: 12,
    };
    let target = list
        .head
        .as_ref()
        .unwrap()
        .clone()
        .borrow()
        .next
        .as_ref()
        .unwrap()
        .clone();
    let target2 = target.borrow().next.as_ref().unwrap().clone();
    list.append_after(target, dawa);
    list.append_after(target2, a);
    list.print_list();
    println!("list len: {}", list.list_len());
}

#[derive(Debug)]
pub struct Node<T: Display> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

#[derive(Debug, Default)]
pub struct List<T: Display> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T: Display> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }))
    }
}

impl<T: Display> List<T> {
    pub fn list_len(&self) -> usize {
        self.len
    }

    pub fn print_list(&self) {
        // print the data from all nodes one perline
        if let Some(head_node) = &self.head {
            let mut current_node = head_node.clone();
            println!("{}", current_node.borrow().data);
            while current_node.borrow().next.is_some() {
                let next_node = current_node.borrow().next.as_ref().unwrap().clone();
                current_node = next_node.clone();
                println!("{}", current_node.borrow().data);
            }
        } else {
            println!("Empty List!!");
        }
    }

    pub fn append_node_behind(&mut self, data: T) {
        let new_node = Node::new(data);
        // habdle empty List-> new node becomes head and tail
        if let Some(tail_node) = &self.tail {
            tail_node.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(Rc::downgrade(tail_node));
            self.tail = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        }
        self.len += 1;
    }

    pub fn append_node_front(&mut self, data: T) {
        let new_node = Node::new(data);
        // habdle empty List-> new node becomes head and tail
        if let Some(head_node) = &self.head {
            head_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
            new_node.borrow_mut().next = Some(head_node.clone());
            self.head = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node.clone());
        }
        self.len += 1;
    }

    pub fn append_after(&mut self, target: Rc<RefCell<Node<T>>>, data: T) {
        if let Some(head_node) = &self.head {
            assert!(self.tail.is_some());
            let new_node = Node::new(data);
            // if target doesn't have prev it must be the head node
            if target.borrow().prev.is_none() {
                assert!(Rc::ptr_eq(head_node, &target));
                if target.borrow().next.is_some() {
                    let next_node = target.borrow().next.as_ref().unwrap().clone();
                    next_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                    new_node.borrow_mut().next = Some(next_node.clone());
                } else {
                    self.tail = Some(new_node.clone());
                }
                target.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&target));
            }
            // if target doesn't have next it must be the tail node
            else if target.borrow().next.is_none() {
                assert!(Rc::ptr_eq(&target, &self.tail.as_ref().unwrap().clone()));
                new_node.borrow_mut().prev = Some(Rc::downgrade(&target));
                target.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
            // if target has prev and next_node
            else {
                let next_node = target.borrow().next.as_ref().unwrap().clone();
                next_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                target.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&target));
                new_node.borrow_mut().next = Some(next_node.clone());
            }
        } else {
            self.append_node_behind(data);
            return;
        }
        self.len += 1;
    }
}
