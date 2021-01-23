#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let _ = Complex::<f64>::zero();
        assert_eq!(2, 1 + 1);
    }
}

use std::ops::{Add, Mul, Neg, AddAssign, SubAssign};

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0f64
    }
}

impl Zero for f32 {
    fn zero() -> Self {
        0.0f32
    }
}

const COMPLEX : Clifford = Clifford {
    positive: 1,
    negative: 1,
    zero: 0,
};

pub type Complex<T> = Multivector<T, COMPLEX>;

const fn is_canonically_ordered(mut lhs: usize, rhs: usize) -> bool {
    lhs <<= 1;

    let mut sum = 0usize;
    while lhs != 0 {
        sum += usize::count_ones(lhs & rhs) as usize;
        lhs <<= 1;
    }
    sum % 2 == 0
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Clifford {
    positive: usize,
    negative: usize,
    zero: usize
}

impl Clifford {
    const fn dim(self) -> usize {
        self.positive + self.negative + self.zero
    }
    const fn size(self) -> usize {
        1 << self.dim()
    }
}

pub struct Multivector<T, const C: Clifford> where
[(); C.size()]: Sized,
{
    data: [T; C.size()],
}

impl<T, const C: Clifford> Zero for Multivector<T, C> where
T: Copy + Zero,
[(); C.size()]: Sized,
{
    fn zero() -> Self {
        Self {
            data: [T::zero(); C.size()],
        }

    }
}

impl<T, const C: Clifford> Multivector<T, C> where
[(); C.size()]: Sized,
{
    pub fn inner_product(&self, other: &Self) -> T where
    T: Clone + AddAssign + SubAssign + Mul<T, Output = T> + Zero,
    {
        let mut x = T::zero();
        for i in 0..C.positive {
            let j = 1 << i;
            x += self.data[j].clone() * other.data[j].clone();
        }
        for i in C.positive..(C.positive + C.negative) {
            let j = 1 << i;
            x -= self.data[j].clone() * other.data[j].clone();
        }
        x
    }

    pub fn outer_product(&self, other: &Self) -> Self where
    T: Clone + Mul<T, Output = T> + Neg<Output = T> + AddAssign + SubAssign,
    Multivector<T, C>: Zero,
    {
        let mut x = Self::zero();
        for i in 0..C.size() {
            for j in 0..C.size() {
                let val = self.data[i].clone() * other.data[i].clone();
                x.data[i ^ j] += if is_canonically_ordered(i, j) {
                    val
                } else {
                    val.neg()
                }
                                    
            }
        }
        x

    }

}

impl<T, const C: Clifford> Mul for Multivector<T, C> where
[(); C.size()]: Sized,
T: Copy + AddAssign + SubAssign + Mul<T, Output = T> + Neg<Output = T> + Zero,
Multivector<T, C>: Add<T, Output = Multivector<T, C>>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        self.outer_product(&other) + self.inner_product(&other)
    }
}

impl<T, const C: Clifford> Add for Multivector<T, C> where
[(); C.size()]: Sized,
T: Clone + AddAssign,
{
    type Output = Self;
    fn add(mut self, other: Self) -> Self::Output {
        for i in 0..C.size() {
            self.data[i] += other.data[i].clone();
        }
        self
    }
}

impl<T, const C: Clifford> Add<T> for Multivector<T, C> where
[(); C.size()]: Sized,
T: Clone + AddAssign,
{
    type Output = Self;
    fn add(mut self, other: T) -> Self::Output {
        self.data[0] += other;
        self
    }
}
