use std::fmt::{Display, Debug};

// INSERTION SORT
// Ave: O(n^2), Best: O(n), Worst: O(n^2)
//
// Example: 1 7 3 2 8 5
// 1 (i) 7 2 3 8 5, i = index
// 1 7 (i) 2 3 8 5
// 1 2 (i) 7 3 8 5 <- swapped 1, 2
// 1 2 7 (i) 3 8 5
// 1 2 3 (i) 7 8 5 <- swapped 2, 3
// 1 2 3 7 (i) 8 5
// 1 2 3 7 8 (i) 5
// 1 2 3 5 7 (i) 8 <- swapped 3, 5 (<- compare 3, 4 <- compare 4, 5)
pub fn insertion_sort<T>(src: &mut [T])
    where T: PartialOrd + Copy + Display + Debug
{
    
    if src.len() <= 1 {
        return;
    }

    for i in 0..src.len() {
        let mut j = i;

        // back and compare 
        // `src[j] < src[j-1]` is important
        while j > 0 && src[j] < src[j-1] {
            src.swap(j, j-1);
            j -= 1;
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sort::test_utils::test_sort;

    #[test]
    fn test_insertion() {
        test_sort(Box::new(insertion_sort), 10);
    }
}
