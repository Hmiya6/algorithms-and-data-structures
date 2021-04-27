use std::fmt::{Display, Debug};


// 2 5 1 8 7 3(limit)  
// 2 1 5 7 3(limit) 8
// 1 2 5 3(limit) 7 8
// 1 2 3(limit) 5 7 8
//
// 1. limit を 1つずつ前にすすめる
// 2. 0..limit で 2つの連続した要素を順に比べて必要ならスワップ 
//  -> 0..limit 間の最も大きい要素は limit の後ろにくる (ソート済みになる)
pub fn bubble_sort<T: PartialOrd + Copy + Display + Debug>(slice: &mut [T]) {
    for limit in (0..slice.len()).rev() {
        for i in 0..limit {
            if slice[i] > slice[i+1] {
                slice.swap(i, i+1);
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rand;

    #[test]
    fn test_bubble() {
        let mut slice = [1, 4, 3, 2, 1];
        bubble_assert(&mut slice, &[1, 1, 2, 3, 4]);

        let mut num = [0];
        bubble_assert(&mut num, &[0]);

        let mut random = rand::random::<[i32; 10]>();
        let mut sorted = random.clone();
        sorted.sort();
        bubble_assert(&mut random, &sorted);
    }

    fn bubble_assert<T: PartialOrd + Copy + Display + Debug>(slice: &mut [T], expected: &[T]) {
        bubble_sort(slice);
        assert_eq!(slice, expected);
    }
}
