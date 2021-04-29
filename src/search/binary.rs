
use std::cmp::Ordering;

pub fn binary_search<T: Ord>(item: &T, src: &[T]) -> Option<usize> {
    
    let mut left = 0; // inclusive
    let mut right = src.len(); // exclusive

    while left < right {
        let mid = (left + right)/2;
        
        match item.cmp(&src[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => left = mid + 1,
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    
    #[test]
    fn search() {
        let mut rng = thread_rng();

        // search a random index from 10 length vec, repeat 100 times
        for _ in 0..100 {
            let rand_index: usize = rng.gen_range(0..10);
            let mut rand_vec: Vec<i32> = (0..10).map(|_| {
                rng.gen::<i32>()
            }).collect();
            rand_vec.sort();
            assert_eq!(binary_search(&rand_vec[rand_index], &rand_vec), Some(rand_index));
        }
    }
}
