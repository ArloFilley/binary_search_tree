use std::vec;

fn main() {
    let mut binary_search_tree = Node {
        value: 5,
        left: None,
        right: None
    };
    binary_search_tree.insert(4);
    binary_search_tree.insert(3);
    binary_search_tree.insert(7);
    binary_search_tree.insert(9);
    binary_search_tree.insert(6);
    binary_search_tree.insert(10);

    binary_search_tree.level_order();
    binary_search_tree.post_order();
    println!("");
    binary_search_tree.pre_order();
    println!("");
    binary_search_tree.in_order(); 
    println!("");
}

#[derive(Clone)]
pub struct Node {
    pub value: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>
}


impl Node {
    pub fn insert(
        &mut self,
        value: u32
    ) {
        if value < self.value {
            match self.left {
                None => self.left = Some(Box::new(Node {
                    value: value,
                    left: None,
                    right: None
                })),
                Some(ref mut node) => node.insert(value)
            }
        } else {
            match self.right {
                None => self.right = Some(Box::new(Node {
                    value: value,
                    left: None,
                    right: None
                })),
                Some(ref mut node) => node.insert(value)
            }
        }
    }

    pub fn level_order(
        &self
    ) {
        let mut queue: Vec<Node> = vec![];
        queue.push(self.clone());
        while queue.len() != 0 {
            print!("{},", queue[0].value);
    
            if queue[0].left.is_some() {
                queue.push(*queue[0].clone().left.unwrap())
            }
    
            if queue[0].right.is_some() {
                queue.push(*queue[0].clone().right.unwrap())
            }
    
            queue.remove(0);
        }
        println!(""); 
    }

    pub fn post_order(
        & self
    ) {
        match self.left {
            Some(ref node) => node.post_order(),
            None => print!(""),
        }
   
        match self.right {
            Some(ref node) => node.post_order(),
            None => print!(""),
        }

        print!("{},", self.value); 
    }

    pub fn pre_order(
        & self
    ) {
        print!("{},", self.value); 

        match self.left {
            Some(ref node) => node.pre_order(),
            None => print!(""),
        }
   
        match self.right {
            Some(ref node) => node.pre_order(),
            None => print!(""),
        }
    }

    pub fn in_order(
        & self
    ) {
        match self.left {
            Some(ref node) => node.in_order(),
            None => print!(""),
        }
        
        print!("{},", self.value);
   
        match self.right {
            Some(ref node) => node.in_order(),
            None => print!(""),
        }
    }
}