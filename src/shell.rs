use std::fmt::{Display, Debug};


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
    use crate::test_utils::test_sort;

    #[test]
    fn test_shell() {
        test_sort(Box::new(shell_sort), 10);
    }
}

