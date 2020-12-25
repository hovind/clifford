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

pub struct Multivector<T, const N: usize> where
[(); 1 << N]: Sized,
{
    data: [T; 1 << N],
}

impl<T, const N: usize> Multivector<T, N> where
[(); 1 << N]: Sized,
{
    pub fn grade<const G: usize>(&self) -> &[T; choose(N, G)] {
        let offset = 1 << G;
        unsafe {
            std::ptr::read(&self.data[offset..] as *const [T] as *const &[T; choose(N, G)])
        }
    }
}
