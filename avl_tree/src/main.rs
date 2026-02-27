
#[derive(Debug, Clone)]
struct AVLTree {
    value: i32,
    left: Option<Box<AVLTree>>,
    right: Option<Box<AVLTree>>,
    height: i32,
    marked_for_deletion: bool,
}

impl AVLTree{
    fn new(value: i32) -> Self {
        AVLTree {
            value,
            left: None,
            right: None,
            height: 1,
            marked_for_deletion: false,
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


        self.upd_height_and_balance();

    }

    fn _delete_left(&mut self) {
        self.left = None;
        self.height = 1 + AVLTree::height_of(&self.left).max(AVLTree::height_of(&self.right)); 
   
    }

    fn _delete_right(&mut self) {
        self.right = None;
        self.height = 1 + AVLTree::height_of(&self.left).max(AVLTree::height_of(&self.right)); 
               
    }

    fn delete(&mut self, value: i32) -> bool { // true if deleted, false if not found
        let mut deleted = false;

        if value < self.value {
            if let Some(left) = &mut self.left {
                deleted = left.delete(value) || deleted;
            } else {
                return false; // value not found
            }
        } else if value > self.value {
            if let Some(right) = &mut self.right {
                deleted = right.delete(value) || deleted;
            } else {
                return false; // value not found
            }
        } else {// value is the value, delete self
            self._delete_current_node();
            return true;
        };

        if let Some(left) = &mut self.left {
            if left.marked_for_deletion {
                self._delete_left();
                deleted = true;
            }
        }else if let Some(right) = &mut self.right {
            if right.marked_for_deletion {
                self._delete_right();
                deleted = true;
            }
        }
    
        if deleted {
            self.upd_height_and_balance();
        }
        
        deleted
    }

    fn upd_height_and_balance(&mut self) {
        self.height = 1 + AVLTree::height_of(&self.left).max(AVLTree::height_of(&self.right));
        let balance_factor = self.balance_factor();
        if balance_factor > 1 {
            if self.left.as_ref().unwrap().balance_factor() >= 0 {
                // left left
                self.right_rotate();
            } else {
                //left right
                self.left.as_mut().unwrap().left_rotate();
                self.right_rotate();
            }
        }else if balance_factor < -1 {
            if self.right.as_ref().unwrap().balance_factor() <= 0 {
                // right right
                self.left_rotate();
            } else {
                // right left
                self.right.as_mut().unwrap().right_rotate();
                self.left_rotate();
            }
        }
    }
        

    fn find_min(&self) -> i32 {
        match &self.left {
            Some(left) => left.find_min(),
            None => self.value,
        }
    }

    fn _delete_current_node(&mut self) {

    match (&self.left, &self.right) {
        (None, None) => {
            *self = AVLTree::new(0);
            self.marked_for_deletion = true;
        }
        (None, Some(_)) => {
            *self = *self.right.take().unwrap();
        }
        (Some(_), None) => {
            *self = *self.left.take().unwrap();
        }
        (Some(_), Some(_)) => {
            let successor_value = self.right.as_ref().unwrap().find_min();
            self.value = successor_value;
            self.right.as_mut().unwrap().delete(successor_value);
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
    println!("AVL tree test: inserting a bunch of elements");
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

    println!("Deleting some elements");
    tree.delete(20);
    tree.delete(20);
    tree.delete(50);
    tree.pprint(0);

    
}
