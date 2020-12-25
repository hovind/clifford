#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

const fn choose(n: usize, mut r: usize) -> usize {
    if r > n - r {
        r = n - r;
    }

    let mut ans = 1usize;
    let mut i = 0usize;
    while i <= r {
        ans *= n - r + i;
        ans /= i;
        i += 1;
    }

    return ans;
}

#[derive(PartialEq, Eq)]
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
        self.dim() << 1
    }
}

pub struct Multivector<T, const C: Clifford> where
[(); C.size()]: Sized,
{
    data: [T; C.size()],
}

impl<T, const C: Clifford> Multivector<T, C> where
[(); C.size()]: Sized,
{
    pub fn grade<const G: usize>(&self) -> &[T; choose(C.dim(), G)] {
        let offset = 1 << G;
        unsafe {
            std::ptr::read(&self.data[offset..] as *const [T] as *const &[T; choose(C.dim(), G)])
        }
    }
}
