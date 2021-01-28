#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked, const_panic, int_bits_const)]

use clifford::{Clifford, Multivector};
use quickcheck::{Arbitrary, Gen};

#[derive(Clone)]
struct AMultivector<T, const C: Clifford>(Multivector<T, C>) where
[(); C.size()]: Sized;


impl<T, const C: Clifford> Arbitrary for AMultivector<T, C> where
T: Clone + Send + 'static,
[(); C.size()]: Sized,
{
    fn arbitrary<G: Gen>(_: &mut G) -> Self {
        todo!()
    }
}

#[test]
fn test_product() {
    assert_eq!(2 * 2, 4);
}
