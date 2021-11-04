use std::fmt::Display;

type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Debug, Default)]
struct Node<T> {
    value: T,
    left:  NodeRef<T>,
    right: NodeRef<T>,
}

fn generate_tree(level: usize, counter: &mut i32) -> NodeRef<i32> {
    if level == 0 {
        return None
    } else {
        let mut node = Node {
            value: *counter,
            left: None,
            right: None,
        };
        *counter += 1;
        node.left = generate_tree(level - 1, counter);
        node.right =  generate_tree(level - 1, counter);
        Some(Box::new(node))
    }
}

fn print_tree<T: Display>(root: &NodeRef<T>, level: usize) {
    match root {
        Some(node) => {
            print_tree::<T>(&node.left, level + 1);
            for _ in 0..level {
                print!("  ");
            }
            println!("{}", node.value);
            print_tree::<T>(&node.right, level + 1);
        },
        None => {},
    }
}

fn invert_tree<T: Clone>(root: NodeRef<T>) -> NodeRef<T>{
    match root {
        Some(node) => {
            Some(Box::new(Node {
                value: node.value.clone(),
                left: invert_tree::<T>(node.right),
                right: invert_tree::<T>(node.left),
            }))
        },
        None => None
    }
}

fn main() {
    let mut counter = 1;
    let tree = generate_tree(3, &mut counter);
    println!("\n\t* * * Initial binary tree * * *\n");
    print_tree(&tree, 0);
    println!("\n\t* * * Inverted binary tree * * *\n");
    print_tree(&invert_tree(tree), 0)
}