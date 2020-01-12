use crate::Matrix;
use crate::Scalar;
use staticvec::StaticVec;

// Matrix<M, N> * Scalar = Matrix<M, N>
impl<const M: usize, const N: usize> std::ops::Mul<Scalar> for &Matrix<M, N> {
    type Output = Matrix<M, N>;
    fn mul(self, rhs: Scalar) -> Self::Output {
        let mut out = self.clone();

        for m in 0..M {
            for n in 0..N {
                out[m][n] = self[m][n] * rhs;
            }
        }

        out
    }
}

// Matrix<M, N> - Matrix<M, N> = Matrix<M, N>
impl<const M: usize, const N: usize> std::ops::Sub<&Matrix<M, N>> for &Matrix<M, N> {
    type Output = Matrix<M, N>;
    fn sub(self, rhs: &Matrix<M, N>) -> Self::Output {
        let mut out = Matrix::<M, N>::default();
        for (m, m_data) in out.data.iter_mut().enumerate() {
            for n in 0..N {
                m_data.insert(n, self[m][n] - rhs[m][n]);
            }
        }
        out
    }
}
// Matrix<M, N> + Matrix<M, N> = Matrix<M, N>
impl<const M: usize, const N: usize> std::ops::Add<&Matrix<M, N>> for &Matrix<M, N> {
    type Output = Matrix<M, N>;
    fn add(self, rhs: &Matrix<M, N>) -> Self::Output {
        let mut out = Matrix::<M, N>::default();
        for (m, m_data) in out.data.iter_mut().enumerate() {
            for n in 0..N {
                m_data.insert(n, self[m][n] + rhs[m][n]);
            }
        }
        out
    }
}

// Matrix<M, N> * Matrix<N, P> = Matrix<M, P>
impl<const M: usize, const N: usize, const P: usize> std::ops::Mul<&Matrix<N, P>>
    for &Matrix<M, N>
{
    type Output = Matrix<M, P>;
    fn mul(self, rhs: &Matrix<N, P>) -> Self::Output {
        let mut out = Matrix::<M, P>::default();
        // Do matrix multiplication
        for (m, m_data) in out.data.iter_mut().enumerate() {
            for p in 0..P {
                let mut f = 0.0;
                for k in 0..M {
                    f += self[m][k] * rhs[k][p];
                }
                m_data.insert(p, f);
            }
        }
        out
    }
}

impl<const M: usize, const N: usize> std::ops::Index<usize> for Matrix<M, N> {
    type Output = StaticVec<Scalar, N>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const M: usize, const N: usize> std::ops::IndexMut<usize> for Matrix<M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
