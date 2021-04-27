
// COUNTING SORT
// Ave/Best/Worst: O(N)
// cons: large memory usage / only uint allowed (-> counting_sort_2)
//
//
// Example: 4 3 6 2 3 4 7
//
// counter -> 0 0 1 2 2 0 1 1
// 
// then,
// ```
// let mut i = 0;
// for (data, &number) in counter.iter().enumerate() {
//    for _ in 0..number {
//        src[i] = data;
//        i += 1;
//    }
// }
// ```
pub fn counting_sort(src: &mut [usize]) {

    let &max = src.iter().reduce(|a, b| {
        if a >= b {a} else {b}
    }).unwrap();

    let mut counter = vec![0 as usize; max+1];
    for &data in src.iter() {
        counter[data as usize] += 1;
    }

    let mut i = 0;
    for (data, &number) in counter.iter().enumerate() {
        for _ in 0..number {
            src[i] = data;
            i += 1;
        }
    }
}


// all int allowed
pub fn counting_sort_2(src: &mut [isize]) {
    let max = *src.iter().reduce(|a, b| {
        if a >= b {a} else {b}
    }).unwrap();

    let min = *src.iter().reduce(|a, b| {
        if a > b {b} else {a}
    }).unwrap();

    let diff = max - min;

    let mut counter = vec![0 as usize; diff.abs() as usize + 1];
    for &data in src.iter() {
        counter[(data - min).abs() as usize] += 1;
    }

    let mut i = 0;
    for (data, &number) in counter.iter().enumerate() {
        for _ in 0..number {
            src[i] = data as isize + min;
            i += 1;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;
    // use crate::test_utils::test_sort;

    #[test]
    fn test_counting() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let mut src: Vec<usize> = [0 as usize; 10].iter()
                .map(|v| v+rng.gen_range(0..1024))
                .collect();
            let mut sorted = src.clone();
            counting_sort(&mut src);
            sorted.sort();
            assert_eq!(src, sorted);
        }

        let mut src = [0];
        counting_sort(&mut src);
        assert_eq!(&src, &[0]);
    }

    #[test]
    fn test_counting_2() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let mut src: Vec<isize> = [0 as isize; 10].iter()
                .map(|v| v-rng.gen_range(0..1024))
                .collect();
            let mut sorted = src.clone();
            counting_sort_2(&mut src);
            sorted.sort();
            assert_eq!(src, sorted);
        }

        let mut src = [0];
        counting_sort(&mut src);
        assert_eq!(&src, &[0]);
    }
}

