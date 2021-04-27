use std::fmt::{Display, Debug};

// MERGE SORT
// Ave/Best/Worst: O(n*log(n))
//
// divide and merge
//
pub fn merge_sort<T>(src: &mut [T])
    where T: PartialOrd + Copy + Display + Debug
{
    if src.len() <= 1 {
        return;
    }

    let mid = src.len()/2;

    let mut left = src[..mid].to_owned();
    merge_sort(&mut left);
    
    let mut right = src[mid..].to_owned();
    merge_sort(&mut right);
    
    merge(src, &left, &right);
}

fn merge<T>(src: &mut [T], left: &[T], right: &[T])
    where T: PartialOrd + Copy + Display + Debug
{
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();
    let mut src_index = 0;

    loop {
        match (left_iter.peek(), right_iter.peek()) {
            (None, None) => break,
            (Some(&&v), None) => {
                src[src_index] = v;
                src_index += 1;
                left_iter.next();
            }
            (None, Some(&&v)) => {
                src[src_index] = v;
                src_index += 1;
                right_iter.next();
            }
            (Some(&&l), Some(&&r)) => {
                if l < r {
                    src[src_index] = l;
                    left_iter.next();
                } else {
                    src[src_index] = r;
                    right_iter.next();
                }
                src_index += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sort::test_utils::test_sort;

    #[test]
    fn test_merge() {
        test_sort(Box::new(merge_sort), 10);
    }
}
