
#[derive(Debug, Clone)]
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

    fn right_rotate(&mut self) {
        if self.left.is_none() {
            return;
        }
        let mut new_root = self.left.take().unwrap();
        self.left = new_root.right.take();
        new_root.right = Some(Box::new(self.clone()));
        *self = *new_root;
    }
    fn left_rotate(&mut self) {
        if self.right.is_none() {
            return;
        }
        let mut new_root = self.right.take().unwrap();
        self.right = new_root.left.take();
        new_root.left = Some(Box::new(self.clone()));
        *self = *new_root;
    }

    fn insert(&mut self, value: i32) {
        // descend
        if value <= self.value {
            match &mut self.left {
                Some(left) => left.insert(value),
                None => self.left = Some(Box::new(AVLTree::new(value))),
            }
        } else if value > self.value {
            match &mut self.right {
                Some(right) => right.insert(value),
                None => self.right = Some(Box::new(AVLTree::new(value))),
            }
        }

        // update height
        self.height = 1 + AVLTree::height_of(&self.left).max(AVLTree::height_of(&self.right)); // 1 + max(left height, right height)
        // balance

        let balance_factor = self.balance_factor();
        if balance_factor > 1 {
            // If the balance factor is greater than 1, then the current node is unbalanced and we are either in the Left Left case or left Right case. To check whether it is left left case or not, compare the newly inserted key with the key in the left subtree root. 
            if value < self.left.as_ref().unwrap().value {
                // Left Left Case
                self.right_rotate();
            } else {
                // Left Right Case
                self.left.as_mut().unwrap().left_rotate();
                self.right_rotate();
            }
        } else if balance_factor < -1 {
            // If the balance factor is less than -1, then the current node is unbalanced and we are either in the Right Right case or Right-Left case. To check whether it is the Right Right case or not, compare the newly inserted key with the key in the right subtree root.    
            if value > self.right.as_ref().unwrap().value {
                // Right Right Case
                self.left_rotate();
            } else {
                // Right Left Case
                self.right.as_mut().unwrap().right_rotate();
                self.left_rotate();
            }
        }

    }

    fn pprint(&self, depth: usize) {
        if let Some(left) = &self.left {
            left.pprint(depth + 1);
             println!("{}{}", "    ".repeat(depth), "/");
        }
       
        println!("{}{}", "    ".repeat(depth), self.value);
       
        if let Some(right) = &self.right {
            println!("{}{}", "    ".repeat(depth), "\\");
            right.pprint(depth + 1);
        }
        
    }

}

fn main() {
    println!("Hello, world!");
    let mut tree = AVLTree::new(10);
    tree.insert(20);
     tree.insert(20);
      tree.insert(20);
       tree.insert(20);

        tree.insert(20);
    tree.insert(30);
    tree.insert(40);
    tree.insert(50);
    tree.insert(25);
    tree.pprint(0);
}
