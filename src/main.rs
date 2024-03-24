use ferris_says::say;
use std::io::{stdout, BufWriter};

mod binary_tree;


fn main() {

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    let _ = say(&message, width, &mut writer);

    const BINARY_TREE_TRAVERSAL_INPUT: [i32; 5] = [2, 5, 1, 3, 4];
    binary_tree::traverse::main(&BINARY_TREE_TRAVERSAL_INPUT, binary_tree::traverse::TraversalType::InOrder);
    binary_tree::traverse::main(&BINARY_TREE_TRAVERSAL_INPUT, binary_tree::traverse::TraversalType::PreOrder);
    binary_tree::traverse::main(&BINARY_TREE_TRAVERSAL_INPUT, binary_tree::traverse::TraversalType::PostOrder);
}
