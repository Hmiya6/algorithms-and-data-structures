
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
    
    // corner case: src.len() == 1
    let mut src = [0];
    f(&mut src);
    assert_eq!(&src, &[0]);
    
    // corner case: src.len() == 0
    let mut src = [];
    f(&mut src);
    assert_eq!(&src, &[]);
}

