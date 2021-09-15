use num::{Signed, Zero};
use std::ops::Add;

pub trait Monoid: Sized {
    fn identity() -> Self;
    fn apply(&self, rhs: &Self) -> Self;
}

pub trait Group: Sized {
    fn identity() -> Self;
    fn inverse(&self) -> Self;
    fn apply(&self, rhs: &Self) -> Self;
}

pub trait Abelian {}

#[derive(Copy, Clone, Debug)]
pub struct Sum<T>(pub T);

impl<T: Copy + Clone + Zero + Add<Output = T>> Monoid for Sum<T> {
    fn identity() -> Self {
        Self(T::zero())
    }

    fn apply(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl<T: Copy + Clone + Add<Output = T> + Zero + Signed> Group for Sum<T> {
    fn identity() -> Self {
        Self(T::zero())
    }

    fn inverse(&self) -> Self {
        Self(-self.0)
    }

    fn apply(&self, rhs: &Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl<T: Add<Output = T>> Abelian for Sum<T> {}

impl<T> From<T> for Sum<T> {
    fn from(x: T) -> Self {
        Sum(x)
    }
}

impl<T: Zero> Default for Sum<T> {
    fn default() -> Self {
        Self(T::zero())
    }
}
