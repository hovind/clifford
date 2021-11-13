#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked, const_panic)]

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn injective_bit_to_blade() {
        const DIM: usize = 8;
        const SIZE: usize = 1 << DIM;
        let mut range = [None; SIZE];
        for i in 0..SIZE {
            let j = bit_to_blade(i, DIM);
            assert_eq!(None, range[j]);
            range[j] = Some(i);
        }
    }

    #[test]
    fn injective_blade_to_bit() {
        const DIM: usize = 8;
        const SIZE: usize = 1 << DIM;
        let mut range = [None; SIZE];
        for i in 0..SIZE {
            let j = blade_to_bit(i, DIM);
            assert_eq!(None, range[j]);
            range[j] = Some(i);
        }
    }

    #[test]
    fn bijective() {
        const DIM: usize = 8;
        const SIZE: usize = 1 << DIM;
        for i in 0..SIZE {
            assert_eq!(i, blade_to_bit(bit_to_blade(i, DIM), DIM));
        }
    }
}

use std::ops::{Add, Mul, Neg, AddAssign, SubAssign};

pub const fn bit_to_blade(x: usize, dim: usize) -> usize {
    let size = 1 << dim;
    let mut n = 0usize;
    let mut i = 0usize;
    while i < size {
        if usize::count_ones(i) < usize::count_ones(x) || i < x && usize::count_ones(i) == usize::count_ones(x) {
            n += 1;
        }
        i += 1;
    }
    n
}

pub const fn blade_to_bit(y: usize, dim: usize) -> usize {
    const fn blade_to_bit_helper(y: usize, dim: usize) -> (usize, usize) {
        let mut i = 0usize;
        let mut c = 1usize;
        let mut base = 0usize;

        while base + c <= y {
            base += c;
            c *= dim - i;
            i += 1;
            c /= i;
        }
        (i, base)
    }

    let (i, base) = blade_to_bit_helper(y, dim);

    let mut k = 0usize;
    let mut j = 0usize;
    while {
        if usize::count_ones(k) as usize == i {
            j += 1;
        }
        base + j <= y
    } {
        k += 1;
    }
    k
}

pub trait Float {
    fn is_nan(&self) -> bool;
}

impl Float for f64 {
    fn is_nan(&self) -> bool {
        f64::is_nan(self.clone())
    }
}

impl Float for f32 {
    fn is_nan(&self) -> bool {
        f32::is_nan(self.clone())
    }
}

pub trait One {
    fn one() -> Self;
}

impl One for f64 {
    fn one() -> Self {
        1.0f64
    }
}

impl One for f32 {
    fn one() -> Self {
        1.0f32
    }
}

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

const STA: Clifford = Clifford {
    positive: 1,
    negative: 3,
    zero: 0,
};

pub const fn vga(d: usize) -> Clifford {
    Clifford {
        positive: d,
        negative: 0,
        zero: 0,
    }
}

pub const fn cga(d: usize) -> Clifford {
    Clifford {
        positive: d,
        negative: 1,
        zero: 0,
    }
}

pub const fn pga(d: usize) -> Clifford {
    Clifford {
        positive: d,
        negative: 0,
        zero: 1,
    }
}

pub type Vga<T, const D: usize> = Multivector<T, { vga(D) }>;
pub type Cga<T, const D: usize> = Multivector<T, { cga(D) }>;
pub type Pga<T, const D: usize> = Multivector<T, { pga(D) }>;
pub type Sta<T> = Multivector<T, STA>;
pub type Hyperbolic<T> = Vga<T, 0>;
pub type Complex<T> = Cga<T, 0>;
pub type Dual<T> = Pga<T, 0>;
pub type Quaternion<T> = Cga<T, 2>;

const fn is_canonically_ordered(mut lhs: usize, rhs: usize) -> bool {
    lhs >>= 1;

    let mut sum = 0u32;
    while lhs != 0 {
        sum += usize::count_ones(lhs & rhs);
        lhs >>= 1;
    }
    sum % 2 == 0
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clifford {
    positive: usize,
    negative: usize,
    zero: usize
}

impl Clifford {
    pub const fn dim(self) -> usize {
        self.positive + self.negative + self.zero
    }
    pub const fn size(self) -> usize {
        1 << self.dim()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl<T, const C: Clifford> Float for Multivector<T, C> where
T: Float,
[(); C.size()]: Sized,
{
    fn is_nan(&self) -> bool {

        for i in 0..C.size() {
            if self.data[i].is_nan() {
                return true;
            }
        }
        return false;
    }
}

impl<T, const C: Clifford> From<[T; C.size()]> for Multivector<T, C> where
[(); C.size()]: Sized,
{
    fn from(data: [T; C.size()]) -> Multivector<T, C> {
        Multivector {
            data: data,
        }
    }
}

impl<T, const C: Clifford> Into<[T; C.size()]> for Multivector<T, C> where
[(); C.size()]: Sized,
{
    fn into(self: Self) -> [T; C.size()] {
        self.data
    }
}

impl<T, const C: Clifford> Multivector<T, C> where
[(); C.size()]: Sized,
{
    pub fn inner_product(&self, other: &Self) -> T where
    T: Clone + AddAssign + SubAssign + Mul<T, Output = T> + Zero,
    {
        let mut x = T::zero();
        x += self.data[0].clone() * self.data[0].clone();
        for i in 1 + C.zero..1 + C.zero + C.positive {
            x += self.data[i].clone() * other.data[i].clone();
        }
        for i in 1 + C.zero + C.positive..1 + C.zero + C.positive + C.negative {
            x -= self.data[i].clone() * other.data[i].clone();
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
                if i & j == 0 {
                    let val = self.data[bit_to_blade(i, C.dim())].clone() * other.data[bit_to_blade(j, C.dim())].clone();
                    x.data[bit_to_blade(i ^ j, C.dim())] += if is_canonically_ordered(i, j) {
                        val
                    } else {
                        val.neg()
                    }
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
