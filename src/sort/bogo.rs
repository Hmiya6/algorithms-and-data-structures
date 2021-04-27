
use rand::prelude::*;
use std::fmt::{Display, Debug};

fn is_sorted<T>(slice: &[T]) -> bool
    where T: PartialOrd {
    
    (0..slice.len()-1).all(|i| {
        slice[i] <= slice[i+1]
    })
}


// shuffle until the slice gets sorted
// ソートされるまでシャッフル
pub fn bogo_sort<T: PartialOrd + Copy + Display + Debug>(slice: &mut [T]) {
    let mut rng = rand::thread_rng();

    while !is_sorted(slice) {
        slice.shuffle(&mut rng);
    }
}




