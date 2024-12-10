/* 

Implement a BTS from vector to integer


*/


struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BinaryTree {
    root: Option<Box<Node>>,
    elements: Vec<Option<Box<Node>>>,
}

/*

In order to use the value of an array as a node we need first of all to wrap th i32 inside of a new node instance, and that will be put inside of a box


*/
impl Node {
    fn new(value: i32) -> Box<Node> {
        Box::new(Node {
            value,
            left: None,
            right: None,
        })
    }

    pub fn get_value(&mut self) -> i32{
        return self.value
    }
}

impl BinaryTree {

    fn insert(&mut self, value: i32) {
        let new_node = Node::new(value);
        self.elements.push(Some(new_node)); // We use Some to avoid to have an empty thing
    }

    fn new_bst_from_array(elements: Vec<i32>) -> BinaryTree {
        let mut tree = BinaryTree { root: None, elements: vec![] };
        for input in elements {
            tree.insert(input);
        }
        tree
    }

    pub fn get_len(&mut self)->usize{
        return self.elements.len()
    }

}

fn main() {
    let elements = vec![1, 2, 3, 4, 5];
    let mut tree = BinaryTree::new_bst_from_array(elements);
    println!("{}", tree.get_len());
}
 