struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    const fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub enum TraversalType {
    InOrder,
    PreOrder,
    PostOrder,
}

fn traverse_in_order(node: Option<Box<Node>>) -> () {
    if node.is_some() {
        let _node = node.unwrap();
        traverse_in_order(_node.left);
        println!("{}", _node.value);
        traverse_in_order(_node.right);
    }
}

fn traverse_pre_order(node: Option<Box<Node>>) -> () {
    if node.is_some() {
        let _node = node.unwrap();
        println!("{}", _node.value);
        traverse_pre_order(_node.left);
        traverse_pre_order(_node.right);
    }
}

fn traverse_post_order(node: Option<Box<Node>>) -> () {
    if node.is_some() {
        let _node = node.unwrap();
        traverse_post_order(_node.left);
        traverse_post_order(_node.right);
        println!("{}", _node.value);
    }
}

fn insert_node(node: &mut Node, value: i32) -> () {
    if value > node.value {
        if node.right.is_none() {
            node.right = Some(Box::new(Node::new(value)));
            return;
        } else {
            insert_node(node.right.as_mut().unwrap(), value);
        }
    } else if value < node.value {
        if node.left.is_none() {
            node.left = Some(Box::new(Node::new(value)));
            return;
        } else {
            insert_node(node.left.as_mut().unwrap(), value);
        }
    } else {
        node.value = value;
    }
}

pub fn main(input: &[i32], traversal_type: TraversalType) -> () {

    if input.len() < 1 {
        return;
    }

    let mut root: Option<Box<Node>> = Some(Box::new(Node::new(input[0])));

    for item in input.iter().enumerate().skip(1) {
        insert_node(root.as_mut().unwrap(), *item.1);
    }

    match traversal_type {
        TraversalType::InOrder => {
            println!("\nIn-order traversal :");
            traverse_in_order(root);
        }
        TraversalType::PreOrder => {
            println!("\nPre-order traversal :");
            traverse_pre_order(root);
        }
        TraversalType::PostOrder => {
            println!("\nPost-order traversal :");
            traverse_post_order(root);
        }
    }
}