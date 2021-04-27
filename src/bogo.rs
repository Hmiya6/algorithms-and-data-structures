
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



// ------------------------------------------------------------
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bogo() {
        let mut nums = [1, 2, 3, 5, 4];
        bogo_assert(&mut nums, &[1, 2, 3, 4, 5]);
        
        let mut num = [1];
        bogo_assert(&mut num, &[1]);

        let mut s = ['a', 'r', 'B'];
        bogo_assert(&mut s, &['B', 'a', 'r']);
    }

    fn bogo_assert<T>(arr: &mut [T], expected: &[T])
    where T: PartialOrd + Copy + Display + Debug {
        bogo_sort(arr);
        assert_eq!(arr, expected);
    }
}

