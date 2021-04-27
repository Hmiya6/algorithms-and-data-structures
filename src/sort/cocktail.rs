use std::fmt::{Display, Debug};
// PartailOrd + Copy + Display + Debug


// COCKTAIL SORT (a.k.a. SHAKER SORT)
//
// Example: 4(s) 5 1 8 7 3(e), (s = start, e = end)
//
// 4(s) 1 5 7 3(e) 8, swapped = true
// 1 4(s) 5 3 7(e) 8, swapped = true
//
// 1 4(s) 3 5(e) 7 8, swapped = true
// 1 3 4(s) 5(e) 7 8, swapped = true
//
// swapped = false -> break
//
//
pub fn cocktail_sort<T>(slice: &mut [T])
    where T: PartialOrd + Copy + Display + Debug {
    
    let mut start = 0;
    let mut end = slice.len();
    let mut swapped = true;

    while swapped {
        swapped = false;

        // start to end
        for i in start..end-1 {
            if slice[i] > slice[i+1] {
                slice.swap(i, i+1);
                swapped = true;
            }
        }
        
        // if not swapped, then end sorting
        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;
        
        // end to start
        for i in (start..end-1).rev() {
           if slice[i] > slice[i+1] {
                slice.swap(i, i+1);
                swapped = true;
            }
        }

        start += 1;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::sort::test_utils::test_sort;

    #[test]
    fn test_cocktail() {
        test_sort(Box::new(cocktail_sort), 10);
    }
}




