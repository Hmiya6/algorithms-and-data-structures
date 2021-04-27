use std::fmt::{Display, Debug};


// SELECTION SORT
//
// Example: 2 5 1 8 7 3
// 2(s) 5 1 8 7 3 (s = start)
// 1 5(s) 2 8 7 3 <- swapped 0, 2
// 1 2 5(s) 8 7 3 <- swapped 1, 2
// 1 2 3 8(s) 7 5 <- swapped 2, 5
// 1 2 3 5 7(s) 8 <- swapped 3, 5
// 1 2 3 5 7 8(s) <- no swap
//
// 
pub fn selection_sort<T>(src: &mut [T])
    where T: PartialOrd + Copy + Display + Debug 
{
    for start in 0..src.len() {
        let mut min_index = start;
        for j in start..src.len() {
            if src[min_index] > src[j] {
                min_index = j;
            }
        }
        
        src.swap(min_index, start);
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::test_sort;

    #[test]
    fn test_selection() {
        test_sort(Box::new(selection_sort), 10);
    }
}
