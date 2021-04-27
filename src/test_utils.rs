
use rand::prelude::*;
pub fn test_sort(f: Box<dyn Fn(&mut [i32])>, times: usize) {
    let mut rng = rand::thread_rng();
    for _ in 0..times {
        let mut src = rng.gen::<[i32; 10]>();
        let mut sorted = src.clone();
        f(&mut src);
        sorted.sort();
        assert_eq!(src, sorted);
    }

}

