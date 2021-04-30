use std::rc::Rc;
use std::cell::RefCell;

pub type Link<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    val: T,
    next: Option<Link<T>>,
    prev: Option<Link<T>>,
}

impl<T> Node<T> {
    
    pub fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }

    pub fn set_next(&mut self, next: Link<T>) {
        self.next = Some(next);
    }

    pub fn get_next(&self) -> Option<Link<T>> {
        match &self.next {
            Some(next) => Some(Rc::clone(next)),
            None => None,
        }
    }

    pub fn set_prev(&mut self, prev: Link<T>) {
        self.prev = Some(prev);
    } 

    pub fn get_prev(&self) -> Option<Link<T>> {
        match &self.prev {
            Some(prev) => Some(Rc::clone(prev)),
            None => None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    length: usize,
}


impl<T> LinkedList<T> {
    
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, item: T) {
        let node = Node::new(item);
        let link = Rc::new(RefCell::new(node));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().set_next(Rc::clone(&link));
            },
            None => {
                self.tail = Some(Rc::clone(&link));
            }
        }

        self.head = Some(Rc::clone(&link));
        self.length += 1;
    }

    pub fn push_back(&mut self, item: T) {
        let node = Node::new(item);
        let link = Rc::new(RefCell::new(node));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().set_next(Rc::clone(&link));
            },
            None =>{
                // if there is not tail, then the pushed node is first node and it should be
                // head and tail.
                self.head = Some(Rc::clone(&link));
            },
        }

        self.tail = Some(Rc::clone(&link));
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                if let Some(next) = head.borrow_mut().next.take() {
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                } else {
                    self.head = None;
                }

                self.length -= 1;
                return Some(
                    Rc::try_unwrap(head).ok()
                        .unwrap()
                        .into_inner()
                        .val
                );
            }
            None => {
                return None;
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(tail) => {
                if let Some(prev) = tail.borrow_mut().prev.take() {
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                } else {
                    self.head = None;
                }
                self.length -= 1;
                return Some(
                    Rc::try_unwrap(tail).ok()
                        .unwrap()
                        .into_inner()
                        .val
                );
            }
            None => {
                return None;
            }
        }
    }
}

