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

pub struct Blade<T, const C: Clifford, const G: usize> where
[(); C.len(G)]: Sized,
{
    data: [T; C.len(G)],
}

impl<T, const C: Clifford> Multivector<T, C> where
[(); C.size()]: Sized,
{
    pub fn blade<const G: usize>(&self) -> &Blade<T, C, G> where
    [(); C.len(G)]: Sized,
    {
        unsafe {
            std::ptr::read(&self.data[C.offset(G)..] as *const [T] as *const &Blade<T, C, G>)
        }
    }
}

impl<T, const C: Clifford> std::ops::Mul for Multivector<T, C> where
[(); C.size()]: Sized,
{
    type Output = Self;
    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
/*
impl<T, const C: Clifford, const GLHS: usize, const GRHS: usize> std::ops::Mul<Blade<T, C, GRHS>> for Blade<T, C, GLHS> where
[(); C.len(GLHS)]: Sized,
[(); C.len(GRHS)]: Sized,
{
    type Output = Blade<T, C, todo!()>;
    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
*/
