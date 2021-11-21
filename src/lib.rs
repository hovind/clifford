#![allow(incomplete_features)]
#![feature(const_for, adt_const_params, generic_const_exprs, iter_zip, maybe_uninit_uninit_array)]

mod multivector;
pub use multivector::{Clifford, Multivector, Float, One, Zero};
use multivector::{STA, vga, cga, pga};


pub type Vga<T, const D: usize> = Multivector<T, { vga(D) }>;
pub type Cga<T, const D: usize> = Multivector<T, { cga(D) }>;
pub type Pga<T, const D: usize> = Multivector<T, { pga(D) }>;
pub type Sta<T> = Multivector<T, STA>;
pub type Hyperbolic<T> = Vga<T, 0>;
pub type Complex<T> = Cga<T, 0>;
pub type Dual<T> = Pga<T, 0>;
pub type Quaternion<T> = Cga<T, 1>;
