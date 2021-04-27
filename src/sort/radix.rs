
// RADIX SORT
// Ave/Best/Worst: O(n)
// uses counting sort
//
// Example: 24 10 11 324 201 101 55
// 
// 1[0] 1[1] 20[1] 10[1] 2[4] 32[4] 5[5]
// 2[0]1 1[0]1 [1]0 [1]1 [2]4 3[2]4 [5]5
// [0]10 [0]11 [0]24 [0]55 [1]01 [2]01 [3]24
//
pub fn radix_sort(src: &mut [usize]) {
    let &max = match src.iter().max() {
        Some(v) => v,
        None => {return;},
    };
    let mut place = 1;
    while place <= max {
        counting_for_radix(src, place);
        place *= 10;
    }
}

fn counting_for_radix(src: &mut [usize], place: usize) {

    let digit_of = |x| x as usize/place % 10;
    
    // count digit occurrences
    let mut counter = vec![0 as usize; 10];
    for &data in src.iter() {
        counter[digit_of(data)] += 1;
    }
    
    // compute last index of each digit
    for i in 1..counter.len() {
        counter[i] += counter[i-1];
    }
    
    // set elements
    for &x in src.to_owned().iter().rev() {
        counter[digit_of(x)] -= 1;
        src[counter[digit_of(x)]] = x;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_radix() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let mut src: Vec<usize> = [0 as usize; 10].iter()
                .map(|v| v+rng.gen_range(0..1024))
                .collect();
            let mut sorted = src.clone();
            radix_sort(&mut src);
            sorted.sort();
            assert_eq!(src, sorted);
        }

        let mut src = [0];
        radix_sort(&mut src);
        assert_eq!(&src, &[0]);
        
        let mut src = [];
        radix_sort(&mut src);
        assert_eq!(&src, &[]);
    }
}

