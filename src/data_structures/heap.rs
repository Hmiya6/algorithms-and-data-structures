

use std::cmp::Ord;

#[derive(Default)]
pub struct Heap<T>
    where T: Ord + Default
{
    count: usize,
    heap: Vec<T>,
}




impl<T> Heap<T>
    where T: Ord + Default
{
    pub fn new() -> Self {
        Self {
            count: 0,
            heap: vec![T::default()],
        }
    }

    pub fn push(&mut self, val: T) {

        self.count += 1;
        self.heap.push(val);
        self.heapify_up(self.count);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.heap.len() == 1 {
            return None;
        }

        let root = std::mem::take(&mut self.heap[1]);
        let data = self.heap.pop().unwrap();

        if self.heap.len() == 1 {
            return Some(root);
        }

        self.heap[1] = data;
        self.count -= 1;
        self.heapify_down(1);

        Some(root)
    }

    fn heapify_up(&mut self, index: usize) {
        let mut index = index;
        while self.parent_index(index) > 0 {
            if self.heap[index] < self.heap[self.parent_index(index)] {
                let parent_index = self.parent_index(index);
                self.heap.swap(index, parent_index);
            }
            index = self.parent_index(index);
        }
    }

    fn heapify_down(&mut self, index: usize) {
        let mut index = index;
        while self.left_child_index(index) <= self.count {
            if self.heap[index] > self.heap[self.min_child_index(index)] {
                let min_child_index = self.min_child_index(index);
                self.heap.swap(index, min_child_index);
            }
            index = self.min_child_index(index);
        }
    }

    fn parent_index(&self, index: usize) -> usize {
        index / 2
    }

    fn right_child_index(&self, index: usize) -> usize {
        index * 2
    }

    fn left_child_index(&self, index: usize) -> usize {
        (index * 2) + 1
    }

    fn min_child_index(&self, index: usize) -> usize {
        if self.right_child_index(index) > self.count {
            return self.left_child_index(index);
        }

        if self.heap[self.left_child_index(index)] < self.heap[self.right_child_index(index)] {
            self.left_child_index(index)
        } else {
            self.right_child_index(index)
        }
    }

}
