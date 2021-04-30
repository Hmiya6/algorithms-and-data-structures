use std::cmp::Ordering;

type Link<T> = Option<Box<BSTNode<T>>>;

pub struct BSTNode<T>
    where T: Ord
{
    val: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> BSTNode<T>
    where T: Ord
{
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn search(&self, val: &T) -> bool {
        match &self.val.cmp(val) {
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
    }

    pub fn insert(&mut self, val: T) {
        if self.val < val {
            match &mut self.left {
                Some(left) => left.insert(val),
                None => self.left = Some(Box::new(BSTNode::new(val))),
            }
        } else {
            match &mut self.right {
                Some(right) => right.insert(val),
                None => self.right = Some(Box::new(BSTNode::new(val))),
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

    fn i64_bst() -> (BSTNode<i64>, Vec<i64>) {
        let mut test_cases = Vec::new();

        let i: i64 = thread_rng().gen();
        test_cases.push(i);
        let mut root = BSTNode::new(i);
        
        for _ in 0..64 {
            let i: i64 = thread_rng().gen();
            test_cases.push(i);
            root.insert(i);
        }
        
        (root, test_cases)
    }

    fn string_bst() -> (BSTNode<String>, Vec<String>) {
        let mut test_cases = Vec::new();

        let s = rand_str();
        test_cases.push(s.clone());
        let mut root = BSTNode::new(s);

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
