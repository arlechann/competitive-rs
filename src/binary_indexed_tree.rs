use crate::group::{Abelian, Group};
use std::{fmt::Debug, ops::RangeBounds};

#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct BIT<T: Abelian + Group> {
    tree: Vec<T>,
}

impl<T: Abelian + Group> BIT<T> {
    pub fn new(n: usize) -> Self {
        Self {
            tree: (0..n).map(|_| T::identity()).collect::<Vec<_>>(),
        }
    }

    pub fn from_slice<U: Clone + Into<T>>(v: &[U]) -> Self {
        Self {
            tree: v.iter().cloned().map(|e| e.into()).collect::<Vec<_>>(),
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    fn up(index: usize) -> usize {
        index + ((index + 1) & !index)
    }

    pub fn add(&mut self, index: usize, value: impl Into<T>) {
        assert!(index < self.len());
        let mut index = index;
        let value = value.into();
        while index < self.len() {
            self.tree[index] = self.tree[index].apply(&value);
            index = Self::up(index);
        }
    }

    pub fn get(&self, index: usize) -> T {
        assert!(index < self.len());
        self.query(index..=index)
    }

    pub fn query(&self, range: impl RangeBounds<usize>) -> T {
        use std::ops::Bound::*;

        let len = self.len();
        let begin = match range.start_bound() {
            Unbounded => 0,
            Included(&b) => b,
            Excluded(&b) => b + 1,
        };
        let end = match range.end_bound() {
            Unbounded => len,
            Included(&e) => e + 1,
            Excluded(&e) => e,
        };
        assert!(begin < end && begin < len && end <= len);
        self.sum(end).apply(&self.sum(begin).inverse())
    }

    fn down(index: usize) -> Option<usize> {
        (index & (index + 1)).checked_sub(1)
    }

    fn sum(&self, end: usize) -> T {
        assert!(end <= self.len());
        let mut ret = T::identity();
        if end == 0 {
            return ret;
        }

        let mut index = end - 1;
        loop {
            ret = ret.apply(&self.tree[index]);
            if let Some(new_index) = Self::down(index) {
                index = new_index;
            } else {
                return ret;
            }
        }
    }
}

#[cfg(test)]
mod test {

    mod bit {
        use super::super::BIT;
        use crate::group::Sum;

        #[test]
        fn test_len() {
            let bit = BIT::<Sum<isize>>::new(0);
            assert_eq!(0, bit.len());
            let bit = BIT::<Sum<isize>>::new(10);
            assert_eq!(10, bit.len());
            let bit = BIT::<Sum<isize>>::new(100000);
            assert_eq!(100000, bit.len());
        }

        #[test]
        fn test_add() {
            let mut bit = BIT::<Sum<isize>>::new(10);
            bit.add(0, 0);
            bit.add(1, 1);
            bit.add(2, 2);
            bit.add(3, 3);
            bit.add(4, 4);
            bit.add(5, 5);
            bit.add(6, 6);
            bit.add(7, 7);
            bit.add(8, 8);
            bit.add(9, 9);
            assert_eq!(0, bit.get(0).0);
            assert_eq!(1, bit.get(1).0);
            assert_eq!(2, bit.get(2).0);
            assert_eq!(3, bit.get(3).0);
            assert_eq!(4, bit.get(4).0);
            assert_eq!(5, bit.get(5).0);
            assert_eq!(6, bit.get(6).0);
            assert_eq!(7, bit.get(7).0);
            assert_eq!(8, bit.get(8).0);
            assert_eq!(9, bit.get(9).0);
        }

        #[test]
        fn test_query() {
            let mut bit = BIT::<Sum<isize>>::new(10);
            bit.add(0, 0);
            bit.add(1, 1);
            bit.add(2, 2);
            bit.add(3, 3);
            bit.add(4, 4);
            bit.add(5, 5);
            bit.add(6, 6);
            bit.add(7, 7);
            bit.add(8, 8);
            bit.add(9, 9);
            assert_eq!(45, bit.query(..).0);
            assert_eq!(10, bit.query(..5).0);
            assert_eq!(35, bit.query(5..).0);
            assert_eq!(27, bit.query(2..8).0);
            assert_eq!(15, bit.query(..=5).0);
            assert_eq!(35, bit.query(2..=8).0);
        }
    }
}
