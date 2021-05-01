use std::cmp::Ordering;

type Link<T> = Option<Box<Node<T>>>;



// TODO: Add `remove` method
pub struct Node<T>
    where T: Ord
{
    val: Option<T>,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T>
    where T: Ord
{
    pub fn new(val: T) -> Self {
        Self {
            val: Some(val),
            left: None,
            right: None,
        }
    }

    pub fn search(&self, val: &T) -> bool {
        match &self.val {
            Some(v) => match v.cmp(val) {
                Ordering::Less => {
                    match &self.left {
                        Some(left) => left.search(val),
                        None => false,
                    }
                }
                Ordering::Equal => {
                    true
                }
                Ordering::Greater => {
                    match &self.right {
                        Some(right) => right.search(val),
                        None => false,
                    }
                }
            }
            None => false,
        }
    }

    pub fn insert(&mut self, val: T) {
        match &self.val {
            Some(v) => {
                if v < &val {
                    match &mut self.left {
                        Some(left) => left.insert(val),
                        None => self.left = Some(Box::new(Node::new(val))),
                    }
                } else {
                    match &mut self.right {
                        Some(right) => right.insert(val),
                        None => self.right = Some(Box::new(Node::new(val))),
                    }
                }
            }
            None => self.val = Some(val),
        }
    }
}

pub struct BinarySearchTree<T>
    where T: Ord
{
    root: Option<Node<T>>,
}

impl<T> BinarySearchTree<T>
    where T: Ord
{
    
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn search(&self, val: &T) -> bool {
        if let Some(root) = &self.root {
            root.search(val)
        } else {
            false
        }
    }

    pub fn insert(&mut self, val: T) {
        match &mut self.root {
            Some(root) => {
                root.insert(val);
            }
            None => {
                let node = Node::new(val);
                self.root = Some(node);
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    use rand::distributions::Alphanumeric;
    
    #[test]
    fn test_bst() {
        let (bst, test_cases) = string_bst();
        for s in test_cases {
            assert!(bst.search(&s));
        }
        assert!(!bst.search(&rand_str()));

        let (bst, test_cases) = i64_bst();
        for i in test_cases {
            assert!(bst.search(&i));
        }
        assert!(!bst.search(&thread_rng().gen::<i64>()));
    }

    fn i64_bst() -> (Node<i64>, Vec<i64>) {
        let mut test_cases = Vec::new();

        let i: i64 = thread_rng().gen();
        test_cases.push(i);
        let mut root = Node::new(i);
        
        for _ in 0..64 {
            let i: i64 = thread_rng().gen();
            test_cases.push(i);
            root.insert(i);
        }
        
        (root, test_cases)
    }

    fn string_bst() -> (Node<String>, Vec<String>) {
        let mut test_cases = Vec::new();

        let s = rand_str();
        test_cases.push(s.clone());
        let mut root = Node::new(s);

        for _ in 0..20 {
            let s = rand_str();
            test_cases.push(s.clone());
            root.insert(s);
        }
        (root, test_cases)
    }

    fn rand_str() -> String {
        thread_rng().sample_iter(Alphanumeric)
            .take(20)
            .map(char::from)
            .collect::<String>()
    }
}
