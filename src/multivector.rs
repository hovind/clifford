use core::ops::{Add, Mul, Neg, AddAssign, SubAssign};
use core::iter::{zip};

#[cfg(test)]
mod tests;


pub trait Float {
    fn is_nan(&self) -> bool;
    fn is_infinite(&self) -> bool;
}

impl Float for f64 {
    fn is_nan(&self) -> bool {
        f64::is_nan(self.clone())
    }
    fn is_infinite(&self) -> bool {
        f64::is_infinite(self.clone())
    }
}

impl Float for f32 {
    fn is_nan(&self) -> bool {
        f32::is_nan(self.clone())
    }
    fn is_infinite(&self) -> bool {
        f32::is_infinite(self.clone())
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    pub const fn negative_bits(self) -> usize {
        (1 << self.negative) - 1 << self.positive
    }

    pub const fn zero_bits(self) -> usize {
        (1 << self.zero) - 1 << self.positive + self.negative
    }

    const fn zero_by_form(self, x: usize) -> bool {
        usize::count_ones(self.zero_bits() & x) != 0
    }

    const fn flip_by_form(self, x: usize) -> bool {
        usize::count_ones(self.negative_bits() & x) % 2 != 0
    }

    const fn flip_by_anticommutativity(mut lhs: usize, rhs: usize) -> bool {
        lhs >>= 1;

        let mut flips = 0u32;
        while lhs != 0 {
            flips += usize::count_ones(lhs & rhs);
            lhs >>= 1;
        }
        flips % 2 != 0
    }

    const fn bit_to_blade(self, x: usize) -> usize {
        let mut n = 0usize;
        let mut i = 0usize;
        while i < self.size() {
            if usize::count_ones(i) < usize::count_ones(x) || i < x && usize::count_ones(i) == usize::count_ones(x) {
                n += 1;
            }
            i += 1;
        }
        n
    }

    const fn blade_to_bit(self, y: usize) -> usize {
        const fn blade_to_bit_helper(dim: usize, y: usize) -> (usize, usize) {
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

        let (i, base) = blade_to_bit_helper(self.dim(), y);

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
        self.data.iter().any(Float::is_nan)
    }
    fn is_infinite(&self) -> bool {
        self.data.iter().any(Float::is_infinite)
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
    pub fn inner_product<'a, 'b>(&'a self, other: &'b Self) -> T where
    T: Clone + AddAssign + SubAssign + Zero,
    &'a T: Mul<&'b T, Output = T>,
    {
        let mut v = T::zero();
        for (i, (x, y)) in zip(&self.data, &other.data).enumerate() {
            let j = C.blade_to_bit(i);
            if C.zero_by_form(j) {
                continue;
            } else if C.flip_by_form(j) {
                v -= x * y;
            } else {
                v += x * y;
            }
        }
        v
    }

    pub fn outer_product<'a, 'b> (&'a self, other: &'b Self) -> Self where
    T: Clone + AddAssign + Neg<Output = T>,
    &'a T:  Mul<&'b T, Output = T>,
    Multivector<T, C>: Zero,
    {
        let mut x = Self::zero();
        for i in 0..C.size() {
            for j in 0..C.size() {
                let k = i ^ j;
                if i != j && !C.zero_by_form(k) {
                    let lhs = &self.data[C.bit_to_blade(i)];
                    let rhs = &other.data[C.bit_to_blade(j)];
                    let val = lhs * rhs;
                    let flip = Clifford::flip_by_anticommutativity(i , j) != C.flip_by_form(i & j);
                    x.data[C.bit_to_blade(k)] += if flip {
                        val.neg()
                    } else {
                        val
                    };

                }
            }
        }
        x
    }

}

impl<'a, 'b, T, const C: Clifford> Mul<&'b Multivector<T, C>> for &'a Multivector<T, C> where
[(); C.size()]: Sized,
T: Copy + AddAssign + Neg<Output = T> + SubAssign + Zero,
&'a T: Mul<&'b T, Output = T>,
Multivector<T, C>: Add<T, Output = Multivector<T, C>>,
{
    type Output = Multivector<T, C>;
    fn mul(self, other: &'b Multivector<T, C>) -> Self::Output {
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

pub const STA: Clifford = Clifford {
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
