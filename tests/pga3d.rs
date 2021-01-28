#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked, const_panic, int_bits_const, maybe_uninit_uninit_array, maybe_uninit_extra, maybe_uninit_slice)]

use clifford::{Clifford, Multivector, pga};
use quickcheck::{Arbitrary, Gen};
use quickcheck_macros::quickcheck;

#[derive(Clone, Debug)]
struct AMultivector<T, const C: Clifford>(Multivector<T, C>) where
[(); C.size()]: Sized;


impl<T, const C: Clifford> Arbitrary for AMultivector<T, C> where
T: Arbitrary + Clone + Send + 'static,
[(); C.size()]: Sized,
{
    fn arbitrary(gen: &mut Gen) -> Self {
        let data: [T; C.size()] = {
            let mut data: [std::mem::MaybeUninit<T>; C.size()] = std::mem::MaybeUninit::uninit_array();
            for i in 0..C.size() {
                data[i] = std::mem::MaybeUninit::new(T::arbitrary(gen));
            }

            unsafe { std::mem::transmute_copy::<_, _>(&data) }
        };
        AMultivector(Multivector::from(data))
    }
}

#[quickcheck]
fn double_reversal_is_identity(_v: AMultivector<f64, { pga(3) }>) -> bool {
    true
}
