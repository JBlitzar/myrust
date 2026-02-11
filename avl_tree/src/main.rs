
struct AVLTree {
    value: i32,
    left: Option<Box<AVLTree>>,
    right: Option<Box<AVLTree>>,
    height: i32,
}

impl AVLTree{
    fn new(value: i32) -> Self {
        AVLTree {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }

    fn height_of(node: &Option<Box<AVLTree>>) -> i32 {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }
    fn balance_factor(&self) -> i32 {
        let lh = AVLTree::height_of(&self.left);
        let rh = AVLTree::height_of(&self.right);
        lh - rh
    }

    fn insert(&mut self, value: i32) {
        // descend

        // update height

        // balance

    }

}

fn main() {
    println!("Hello, world!");
}
