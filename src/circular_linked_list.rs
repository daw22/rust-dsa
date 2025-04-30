use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
pub struct CircularList {
    head: Option<Rc<RefCell<Node>>>,
    len: usize,
}

pub fn run_tests() {
    println!("Circular linked list tests");
    let mut list = CircularList::new();
    list.append_front(1);
    list.append_behind(2);
    list.append_before_val(10, 2);
    list.append_after_val(20, 2);
    list.print_list();
    let node = list.head.as_ref().unwrap().clone();
    println!("list len: {}", list.list_len());
    println!("after last node: {}", list.get_next_val(node));
}
impl Node {
    fn new(x: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val: x, next: None }))
    }
}

impl CircularList {
    fn new() -> CircularList {
        Self { head: None, len: 0 }
    }

    fn list_len(&self) -> usize {
        self.len
    }

    fn print_list(&self) {
        if let Some(head_node) = &self.head {
            println!("{}", head_node.borrow().val);
            let mut current_node = head_node.borrow().next.as_ref().unwrap().clone();
            while !Rc::ptr_eq(&current_node, head_node) {
                println!("{}", current_node.borrow().val);
                let next_node = current_node.borrow().next.as_ref().unwrap().clone();
                current_node = next_node;
            }
        } else {
            println!("Empity list!!");
        }
    }
    fn append_behind(&mut self, value: i32) {
        let new_node = Node::new(value);
        if let Some(head_node) = &self.head {
            // make the new_node point to the head_node
            let mut new_node_mut = new_node.borrow_mut();
            new_node_mut.next = Some(head_node.clone());
            // get the tail node and make it point to the new_node
            let mut current_node = head_node.clone();
            // let mut tail_node = tail_borrow.next.as_ref().unwrap().clone();
            while !Rc::ptr_eq(current_node.borrow().next.as_ref().unwrap(), &head_node) {
                let next_node = current_node.borrow().next.as_ref().unwrap().clone();
                current_node = next_node;
                //detect if list is broken (no node pointing to head)
                if Rc::ptr_eq(&current_node, head_node) {
                    panic!("broken link detected");
                }
            }
            current_node.borrow_mut().next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
            let mut new_node_mut = new_node.borrow_mut();
            new_node_mut.next = Some(new_node.clone());
        }

        self.len += 1;
    }

    fn append_front(&mut self, val: i32) {
        let new_node = Node::new(val);
        if let Some(head_node) = &self.head {
            // find the tail node and make it point to the new node
            let mut current_node = head_node.clone();
            while !Rc::ptr_eq(&current_node.borrow().next.as_ref().unwrap(), head_node) {
                let next_node = current_node.borrow().next.as_ref().unwrap().clone();
                current_node = next_node;
            }
            current_node.borrow_mut().next = Some(new_node.clone());
            // make new node point to the head_node(old)
            new_node.borrow_mut().next = Some(head_node.clone());
            // change list's head with the new node
            self.head = Some(new_node.clone());
        } else {
            //empity list, make new node the head node
            new_node.borrow_mut().next = Some(new_node.clone());
            self.head = Some(new_node.clone());
        }
        self.len += 1;
    }

    fn append_after_val(&mut self, val: i32, after: i32) {
        // handle empty list
        if let Some(head_node) = &self.head {
            // handle after = head.val
            if head_node.borrow().val == after {
                self.append_front(val);
                //self.len += 1;
                return;
            }
            // find node with val = after
            let mut current_node = head_node.clone();
            while current_node.borrow().val != after {
                let next_node = current_node.borrow().next.as_ref().unwrap().clone();
                current_node = next_node;
                // if head node reached before finding val append as tail and exit
                if Rc::ptr_eq(&current_node, head_node) {
                    self.append_behind(val);
                    return;
                }
            }
            // insert new node after it
            let new_node = Node::new(val);
            let node_after_current = current_node.borrow().next.as_ref().unwrap().clone();
            // new node points to node_after_current_node
            new_node.borrow_mut().next = Some(node_after_current.clone());
            // current node points to new node
            current_node.borrow_mut().next = Some(new_node.clone());
        } else {
            self.append_front(val);
        }
        self.len += 1;
    }

    fn append_before_val(&mut self, val: i32, before: i32) {
        // handle empty list
        if let Some(head_node) = &self.head {
            // handle after = head.val
            if head_node.borrow().val == before {
                self.append_front(val);
                //self.len += 1;
                return;
            }
            // find node with val = before
            let mut current_node = head_node.clone();
            while current_node.borrow().val != before {
                let next_node = current_node.borrow().next.as_ref().unwrap().clone();
                if next_node.borrow().val != before {
                    current_node = next_node;
                } else {
                    break;
                }
                // if head node reached before finding val append as tail and exit
                if Rc::ptr_eq(&current_node, head_node) {
                    self.append_behind(val);
                    return;
                }
            }
            // insert new node after curent node(before the target node)
            let new_node = Node::new(val);
            let target_node = current_node.borrow().next.as_ref().unwrap().clone();
            // new node points to target_node
            new_node.borrow_mut().next = Some(target_node.clone());
            // current node points to new node
            current_node.borrow_mut().next = Some(new_node.clone());
        } else {
            self.append_front(val);
        }
        self.len += 1;
    }

    fn get_next_val(&self, node: Rc<RefCell<Node>>) -> i32 {
        let next_node = node.borrow().next.as_ref().unwrap().clone();
        let x = next_node.borrow().val;
        x
    }

    //fn get_last_node(&self) -> Option<Rc<RefCell<Node>>> {
    //    if let Some(head_node) = &self.head {
    //        let mut current_node = self.head.as_ref().unwrap().clone();
    //        let mut count = 0;
    //        while !Rc::ptr_eq(&current_node, &head_node) {
    //            assert!(count < self.list_len());
    //            let next_node = head_node.borrow().next.as_ref().unwrap().clone();
    //            if Rc::ptr_eq(&next_node, &head_node) {
    //                return Some(current_node);
    //            } else {
    //                current_node = next_node.clone();
    //                count += 1;
    //            }
    //        }
    //    } else {
    //        return None;
    //    }
    //}
}
