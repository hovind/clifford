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
    ans
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
    const fn len(self, grade: usize) -> usize {
        choose(self.dim(), grade)
    }
    const fn offset(self, grade: usize) -> usize {
        let mut g = 0usize;
        let mut ans = 0usize;
        while g < grade {
            ans += self.len(g);
            g += 1;
        }
        ans
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
    pub fn grade<const G: usize>(&self) -> &[T; C.len(G)] {
        let offset = C.offset(G);
        unsafe {
            std::ptr::read(&self.data[offset..] as *const [T] as *const &[T; C.len(G)])
        }
    }
}
