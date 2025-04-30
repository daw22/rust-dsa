pub fn run_tests() {
    let mut tree = BinaryTree::new();
    tree.add_node(3);
    tree.add_node(1);
    tree.add_node(2);
    tree.add_node(4);
    println!("{:?}", tree);
}

// simplebinary tree implementation
#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left_child: Option<Box<TreeNode<T>>>,
    right_child: Option<Box<TreeNode<T>>>,
}

impl<T: PartialEq> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left_child: None,
            right_child: None,
        }
    }
}

#[derive(Debug)]
struct BinaryTree<T> {
    root_node: Option<Box<TreeNode<T>>>,
}

impl<T: PartialEq + PartialOrd> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree { root_node: None }
    }

    fn add_node(&mut self, value: T) {
        let new_node = TreeNode::new(value);
        if self.root_node.is_none() {
            self.root_node = Some(Box::new(new_node));
        } else {
            insert_node(self.root_node.as_mut().unwrap(), new_node);
        }
    }
}

fn insert_node<T: PartialEq + PartialOrd>(root_node: &mut TreeNode<T>, new_node: TreeNode<T>) {
    if root_node.value > new_node.value {
        match &mut root_node.left_child {
            None => root_node.left_child = Some(Box::new(new_node)),
            Some(ref mut node) => insert_node(node, new_node),
        }
    } else {
        match &mut root_node.right_child {
            None => root_node.right_child = Some(Box::new(new_node)),
            Some(ref mut node) => insert_node(node, new_node),
        }
    }
}
