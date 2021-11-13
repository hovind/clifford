#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked, const_panic, maybe_uninit_uninit_array, maybe_uninit_extra, maybe_uninit_slice)]

use clifford::{Clifford, Float, Multivector, Pga, pga};
use quickcheck::{Arbitrary, Gen, QuickCheck};
use pga3d::PGA3D;

#[derive(Clone, Debug, PartialEq, Eq)]
struct AMultivector<T, const C: Clifford>(Multivector<T, C>) where
[(); C.size()]: Sized;

/* TODO: Add alias when it stops generating an ICE, ICE, BABY
 * type APga<T, const D: usize> = AMultivector<T, { pga(D) }>;
 */

impl Into<PGA3D> for AMultivector<f64, { pga(3) }> {
    fn into(self: Self) -> PGA3D {
        PGA3D {
            mvec: self.0.into()
        }
    }
}

impl From<PGA3D> for AMultivector<f64, { pga(3) }> {
    fn from(v: PGA3D) -> Self {
        AMultivector(Multivector::<f64, { pga(3) }>::from(v.mvec))
    }
}


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

impl<T, const C: Clifford> Float for AMultivector<T, C> where
T: Float,
[(); C.size()]: Sized,
{
    fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}


#[test]
fn prop_reference_implementation() {
    fn reference_implementation((u, v): (AMultivector<f64, { pga(3) }>, AMultivector<f64, { pga(3) }>)) -> bool {
        let ours = AMultivector(u.0.clone() * v.0.clone());
        let u_theirs: PGA3D = u.into();
        let v_theirs: PGA3D = v.into();
        let theirs = u_theirs * v_theirs;
        let theirs = AMultivector::<f64, { pga(3) }>::from(theirs);
        ours.is_nan() && theirs.is_nan() || ours == theirs
    }
    QuickCheck::new().quickcheck(reference_implementation as fn((AMultivector<f64, { pga(3) }>, AMultivector<f64, { pga(3) }>)) -> bool);
}
