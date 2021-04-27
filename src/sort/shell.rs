use std::fmt::{Display, Debug};

// SHELL SORT
// Best: O(n*log(n)), Worst: O(n^2)
//
// Example: 5 6 9 2 3
// 5 6 9 2 3 -> gap = len/2 /* == 2 */
// 5 6 9 2 3 <- compared 0, 2 (no swap)
// 5 2 9 6 3 <- swapped 1, 3 (<- compared 1, 3)
// 5 2 3 6 9 <- swapped 2, 4 (<- compared 2, 4)
// 3 2 5 6 9 <- swapped 0, 2
//  -> gap /= 2 /* == 1 */
// 2 3 5 6 9 <- swapped 0, 1
// 2 3 5 6 9 <- compared 1, 2
// 2 3 5 6 9 <- compared 2, 3
// 2 3 5 6 9 <- compared 3, 4
pub fn shell_sort<T>(src: &mut [T])
    where T: PartialOrd + Copy + Display + Debug
{
    let mut gap: usize = src.len() / 2;

    while gap > 0 {
        for i in gap..src.len() {
            let mut j = i;
            while j >= gap && src[j-gap] > src[j] {
                src.swap(j-gap, j);
                j -= gap;
            }
        }
        gap /= 2;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::sort::test_utils::test_sort;

    #[test]
    fn test_shell() {
        test_sort(Box::new(shell_sort), 10);
    }
}

