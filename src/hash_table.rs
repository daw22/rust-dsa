use std::cell::RefCell;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;

pub fn run_tests() {
    let mut table = Table::new(5);
    println!("table capacity: {}", table.collection.len());
    table.add_node("one".to_string(), 1);
    table.add_node("two".to_string(), 2);
    table.add_node("three".to_string(), 3);
    table.add_node("four".to_string(), 4);
    table.add_node("five".to_string(), 5);
    table.add_node("six".to_string(), 6);
    table.add_node("three".to_string(), 333);
    //table.add_node("four".to_string(), 444);
    table.add_node("eight".to_string(), 8);
    table.add_node("seven".to_string(), 7);
    table.add_node("two".to_string(), 22);
    //table.delete("four".to_string());
    table.delete_item("eight".to_string());
    //table.delete_item("six".to_string());
    table.print_list();
    if let Some(val) = table.get_item("three".to_string()) {
        println!("three: {}", val);
    }
    if let Some(val) = table.get_item("four".to_string()) {
        println!("four: {}", val);
    }
    if let Some(val) = table.get_item("six".to_string()) {
        println!("six: {val}");
    }
    if let Some(val) = table.get_item("eight".to_string()) {
        println!("eight: {val}");
    }
}
struct Node<T> {
    key: String,
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(key: String, value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            next: None,
        }))
    }
}

struct Table<T: Display + Clone> {
    capacity: u64,
    collection: Vec<Option<Rc<RefCell<Node<T>>>>>,
}

fn calculate_hash<T: Hash>(payload: T, capacity: u64) -> u64 {
    let mut dh = DefaultHasher::new();
    payload.hash(&mut dh);
    dh.finish() % capacity
}

impl<T: Display + Clone> Table<T> {
    fn new(capacity: u64) -> Table<T> {
        assert!(capacity > 0);
        let mut collection = Vec::with_capacity(capacity as usize);
        for _ in 0..capacity {
            collection.push(None);
        }
        Table {
            capacity,
            collection,
        }
    }

    fn print_list(&self) {
        for i in 0..self.collection.len() {
            let node = self.collection.get(i).unwrap();
            if node.is_some() {
                if node.as_ref().unwrap().clone().borrow().next.is_some() {
                    let mut current_node = node.as_ref().unwrap().clone();
                    print!("{}", current_node.borrow().value);
                    while current_node.borrow().next.is_some() {
                        let next_node = current_node.borrow().next.as_ref().unwrap().clone();
                        current_node = next_node;
                        print!("->{}", current_node.borrow().value);
                    }
                    println!();
                } else {
                    println!("{}", node.as_ref().unwrap().borrow().value);
                }
            }
        }
        println!();
    }
    fn add_node(&mut self, key: String, value: T) {
        let new_node = Node::new(key.clone(), value);
        let index = calculate_hash(key.clone(), self.capacity);
        if self.collection.get(index as usize).unwrap().is_none() {
            self.collection[index as usize] = Some(new_node);
        } else {
            let mut existing_node = self
                .collection
                .get(index as usize)
                .unwrap()
                .as_ref()
                .unwrap()
                .clone();
            if existing_node.borrow().key.as_str() == key.as_str() {
                new_node.borrow_mut().next = existing_node.borrow().next.clone();
                self.collection[index as usize] = Some(new_node);
                return;
            }
            while existing_node.borrow().next.is_some() {
                let next_node = existing_node.borrow().next.as_ref().unwrap().clone();
                if next_node.borrow().key.as_str() == key.as_str() {
                    new_node.borrow_mut().next = next_node.borrow().next.clone();
                    existing_node.borrow_mut().next = Some(new_node);
                    return;
                }
                existing_node = next_node;
            }
            existing_node.borrow_mut().next = Some(new_node);
        }
    }

    fn get_item(&self, key: String) -> Option<T> {
        let index = calculate_hash(key.clone(), self.capacity);
        if let Some(Some(node_rc)) = self.collection.get(index as usize) {
            if node_rc.borrow().key == key.as_str() {
                return Some(node_rc.borrow().value.clone());
            }
            let mut current_rc = node_rc.clone();
            loop {
                if current_rc.borrow().next.is_some() {
                    let next_rc = current_rc.borrow().next.as_ref().unwrap().clone();
                    if next_rc.borrow().key == key.as_str() {
                        return Some(next_rc.borrow().value.clone());
                    }
                    current_rc = next_rc;
                } else {
                    break;
                }
            }
        }
        None
    }

    fn delete_item(&mut self, key: String) -> bool {
        let index = calculate_hash(key.clone(), self.capacity);

        let slot = match self.collection.get_mut(index as usize) {
            Some(s) => s,
            None => return false, // should not happen
        };

        match slot {
            None => return false,
            Some(head_node) => {
                if head_node.borrow().key == key.as_str() {
                    let next_node = head_node.borrow_mut().next.take();
                    *slot = next_node;
                    return true;
                }
            }
        }

        // if the slot is not None but doesn't match the key iterate
        let mut current_node = slot.as_ref().unwrap().clone();
        loop {
            // look ahead
            let mut current_node_borrow = current_node.borrow_mut();
            match current_node_borrow.next {
                Some(ref next_node) if next_node.borrow().key == key.as_str() => {
                    let successor = next_node.borrow_mut().next.take();
                    current_node_borrow.next = successor;
                    return true;
                }
                Some(_) => {
                    let next_node = current_node_borrow.next.as_ref().unwrap().clone();
                    drop(current_node_borrow);
                    current_node = next_node;
                }
                None => {
                    return false;
                }
            }
        }
    }
}
