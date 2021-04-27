use std::fmt::{Display, Debug};

// QUICK SORT
// Ave/Best: O(n*log(n)), Worst: O(n^2)
//
//
pub fn quick_sort<T>(src: &mut [T])
    where T: PartialOrd + Copy + Display + Debug
{
    if src.len() <= 1 {
        return;
    }

    let index = partition(src);
    quick_sort(&mut src[..index]);
    quick_sort(&mut src[index+1..]);
}


fn partition<T>(src: &mut [T]) -> usize
    where T: PartialOrd + Copy + Display + Debug
{
    let mut i: isize = -1; // i represents all of src[<=i] is less than src[pivot]
    let pivot = src.len()-1;

    for j in 0..src.len()-1 {
        if src[j] <= src[pivot] {
            i += 1; // only when src[j] <= src[pivot], i += 1
            src.swap(i as usize, j);
        }
    }

    i += 1;
    src.swap(i as usize, pivot);
    i as usize
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::test_sort;

    #[test]
    fn test_quick() {
        test_sort(Box::new(quick_sort), 10);
    }
}
