use std::fmt::{Display, Debug};

// GNOME SORT
//
// Example: 2 5 1 8 7 3
// 2 5 1 8 7 3 
// 2 1 5 8 7 3 <- swapped 1, 2
// 1 2 5 8 7 3 <- swapped 0, 1 (<- back 1 and compare 0, 1)
// 1 2 5 7 8 3 <- swapped 3, 4
// 1 2 5 7 3 8 <- swapped 4, 5
// 1 2 5 3 7 8 <- swapped 3, 4 (<- back 1 and compare 3, 4)
// 1 2 3 5 7 8 <- swapped 2, 3 (<- back 1 and compare 2, 3)
//
//
pub fn gnome_sort<T>(src: &mut [T])
    where T: PartialOrd + Copy + Debug + Display
{
    let mut index = 0;
    
    if src.len() == 1 {
        return;
    }

    while index < src.len() {
        if index == 0 {
            index += 1;
        }

        if src[index-1] > src[index] {
            // then, swap and back 1.
            src.swap(index-1, index);
            index -= 1;
        } else {
            index += 1;
        }
    }

}


#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::test_sort;

    #[test]
    fn test_gnome() {
        test_sort(Box::new(gnome_sort), 10);
    }
}

