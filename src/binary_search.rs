use std::cmp::{max, min};
use std::ops::*;

pub fn binary_search<T, F>(mut ok: T, mut ng: T, pred: F) -> T
where
    T: Copy + Eq + Ord + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<i8>,
    F: Fn(T) -> bool,
{
    while max(ok, ng) - min(ok, ng) > T::from(1) {
        let middle = (ok + ng) / T::from(2);
        if pred(middle) {
            ok = middle;
        } else {
            ng = middle;
        }
    }
    ok
}

#[cfg(test)]
mod test {
    use super::binary_search;

    #[test]
    fn test_binary_search() {
        assert_eq!(0, binary_search(0, 1000, |x| x <= 0));
        assert_eq!(10, binary_search(0, 1000, |x| x <= 10));
        assert_eq!(100, binary_search(0, 1000, |x| x <= 100));
        assert_eq!(999, binary_search(0, 1000, |x| x <= 1000));
        assert_eq!(1, binary_search(1000, 0, |x| x > 0));
        assert_eq!(11, binary_search(1000, 0, |x| x > 10));
        assert_eq!(101, binary_search(1000, 0, |x| x > 100));
        assert_eq!(1000, binary_search(1000, 0, |x| x > 1000));
    }
}
