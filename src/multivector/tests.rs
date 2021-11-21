use crate::multivector::*;
use std::ops::Rem;
use quickcheck::{Arbitrary, Gen, QuickCheck};

const PGA3: Clifford = pga(3);
const CGA2: Clifford = cga(1);

#[test]
fn injective_bit_to_blade() {
    const C: Clifford = cga(8);
    let mut range = [None; C.size()];
    for i in 0..C.size() {
        let j = C.bit_to_blade(i);
        assert_eq!(None, range[j]);
        range[j] = Some(i);
    }
}

#[test]
fn injective_blade_to_bit() {
    const C: Clifford = cga(8);
    let mut range = [None; C.size()];
    for i in 0..C.size() {
        let j = C.blade_to_bit(i);
        assert_eq!(None, range[j]);
        range[j] = Some(i);
    }
}

#[test]
fn bijective() {
    const C: Clifford = cga(8);
    for i in 0..C.size() {
        assert_eq!(i, C.blade_to_bit(C.bit_to_blade(i)));
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AMultivector<T, const C: Clifford>(Multivector<T, C>) where
[(); C.size()]: Sized;

/* TODO: Add alias when it stops generating an ICE, ICE, BABY
 * type APga<T, const D: usize> = AMultivector<T, { pga(D) }>;
 */

impl Into<ganja::PGA3D> for AMultivector<f64, PGA3> {
    fn into(self: Self) -> ganja::PGA3D {
        let mut x = ganja::PGA3D::zero();
        for i in 0..PGA3.size() {
            x[i] = self.0.data[i];
        }
        return x;
    }
}

impl From<ganja::PGA3D> for AMultivector<f64, PGA3> {
    fn from(v: ganja::PGA3D) -> Self {
        let mut x = Multivector::<f64, PGA3>::zero();
        for i in 0..PGA3.size() {
            x.data[i] = v[i];
        }

        AMultivector(x)
    }
}

impl Into<ganja::QUAT> for AMultivector<f64, CGA2> {
    fn into(self: Self) -> ganja::QUAT {
        let mut x = ganja::QUAT::zero();
        for i in 0..CGA2.size() {
            x[i] = self.0.data[i];
        }
        return x;
    }
}

impl From<ganja::QUAT> for AMultivector<f64, CGA2> {
    fn from(v: ganja::QUAT) -> Self {
        let mut x = Multivector::<f64, CGA2>::zero();
        for i in 0..CGA2.size() {
            x.data[i] = v[i];
        }

        AMultivector(x)
    }
}


impl<T, const C: Clifford> Arbitrary for AMultivector<T, C> where
T: Arbitrary + Clone + Float + One + Rem<T, Output = T> + Send + Zero + 'static,
[(); C.size()]: Sized,
{
    fn arbitrary(gen: &mut Gen) -> Self {
        fn arbitrary_float<T: Arbitrary + Clone + Float + One + Rem<T, Output = T> + Send + Zero + 'static>(gen: &mut Gen) -> T {
            let x = T::arbitrary(gen);
            if x.is_nan() {
                T::zero()
            } else if x.is_infinite() {
                T::one()
            } else {
                x % T::one()
            }
        }
        let data: [T; C.size()] = {
            let mut data: [std::mem::MaybeUninit<T>; C.size()] = std::mem::MaybeUninit::uninit_array();
            for i in 0..C.size() {
                data[i] = std::mem::MaybeUninit::new(arbitrary_float(gen));
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
    fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }
}

#[test]
fn prop_pga3d_implementation() {
    fn reference_implementation((u, v): (AMultivector<f64, PGA3>, AMultivector<f64, PGA3>)) -> bool {
        let ours = AMultivector(&u.0 * &v.0);
        let u_theirs: ganja::PGA3D = u.into();
        let v_theirs: ganja::PGA3D = v.into();
        let theirs = u_theirs * v_theirs;
        let theirs = AMultivector::<f64, PGA3>::from(theirs);
        ours.is_nan() && theirs.is_nan() || ours == theirs
    }
    QuickCheck::new().quickcheck(reference_implementation as fn((AMultivector<f64, PGA3>, AMultivector<f64, PGA3>)) -> bool);
}

#[test]
fn prop_quat_implementation() {
    fn reference_implementation((u, v): (AMultivector<f64, CGA2>, AMultivector<f64, CGA2>)) -> bool {
        let ours = AMultivector(&u.0 * &v.0);
        let u_theirs: ganja::QUAT = u.into();
        let v_theirs: ganja::QUAT = v.into();
        let theirs = u_theirs * v_theirs;
        let theirs = AMultivector::<f64, CGA2>::from(theirs);
        ours.is_nan() && theirs.is_nan() || ours == theirs
    }
    QuickCheck::new().quickcheck(reference_implementation as fn((AMultivector<f64, CGA2>, AMultivector<f64, CGA2>)) -> bool);
}

#[test]
fn simple_quat() {
    let u = AMultivector(Multivector::from([-0.25, -0.0, -0.7, 0.0]));
    let v = AMultivector(Multivector::from([1.0, -0.0, 0.0, 0.3]));
    let ours = AMultivector(&u.0 * &v.0);
    let theirs: ganja::QUAT = Into::<ganja::QUAT>::into(u) * Into::<ganja::QUAT>::into(v);

    let theirs = AMultivector::<f64, CGA2>::from(theirs);
    assert_eq!(ours, theirs)
}

#[test]
fn simple_form() {
    let u = 2;
    let v = 3;
    assert!(CGA2.flip_by_form(u & v))
}
