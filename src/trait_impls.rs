use crate::Matrix;
use crate::Scalar;
use staticvec::StaticVec;
use std::fmt;

impl<const M: usize, const N: usize> fmt::Display for Matrix<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();
        for m in 0..M {
            s.push_str("|");
            for n in 0..N {
                s.push_str(&self[m][n].to_string());
                s.push_str(", ");
            }
            s.push_str("|\n");
        }
        f.write_str(&s)
    }
}

impl<const M: usize, const N: usize> Default for Matrix<M, N> {
    fn default() -> Self {
        let mut data = StaticVec::<StaticVec<Scalar, N>, M>::new();
        for _m in 0..M {
            data.push(StaticVec::<Scalar, N>::new());
        }
        Self { data }
    }
}

impl<const M: usize, const N: usize> From<[[Scalar; N]; M]> for Matrix<M, N> {
    fn from(array: [[Scalar; N]; M]) -> Self {
        let mut data = StaticVec::<StaticVec<Scalar, N>, M>::new();
        for n in array.iter() {
            data.push(StaticVec::<Scalar, N>::new());
            for m in n.iter() {
                data.last_mut().unwrap().push(*m);
            }
        }
        Matrix::<M, N> { data }
    }
}
impl<const M: usize, const N: usize> From<StaticVec<StaticVec<Scalar, N>, M>> for Matrix<M, N> {
    fn from(staticvec: StaticVec<StaticVec<Scalar, N>, M>) -> Self {
        Matrix::<M, N> { data: staticvec }
    }
}

impl<const M: usize, const N: usize> Into<[[Scalar; N]; M]> for Matrix<M, N> {
    fn into(self) -> [[Scalar; N]; M] {
        self.copy_to_array()
    }
}
