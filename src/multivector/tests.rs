use crate::multivector::*;
use quickcheck::{Arbitrary, Gen, QuickCheck};

const PGA3: Clifford = pga(3);

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
fn prop_quat_implementation() {
    fn reference_implementation((u, v): (AMultivector<f64, PGA3>, AMultivector<f64, PGA3>)) -> bool {
        let ours = AMultivector(u.0.clone() * v.0.clone());
        let u_theirs: ganja::PGA3D = u.into();
        let v_theirs: ganja::PGA3D = v.into();
        let theirs = u_theirs * v_theirs;
        let theirs = AMultivector::<f64, PGA3>::from(theirs);
        ours.is_nan() && theirs.is_nan() || ours == theirs
    }
    QuickCheck::new().quickcheck(reference_implementation as fn((AMultivector<f64, PGA3>, AMultivector<f64, PGA3>)) -> bool);
}
