use crate::Matrix;
use crate::Scalar;
use std::mem::MaybeUninit;

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn origin() -> Self {
        let mut out = Matrix::<M, N>::default();
        for m_data in out.data.iter_mut() {
            for n in 0..N {
                m_data.insert(n, 0.0);
            }
        }
        out
    }
    pub fn copy_to_array(&self) -> [[Scalar; N]; M] {
        let mut array: [MaybeUninit<[Scalar; N]>; M] = { MaybeUninit::uninit_array() };
        for i in 0..M {
            unsafe {
                let n = array.get_unchecked_mut(i).as_mut_ptr();
                std::ptr::copy_nonoverlapping(self.data[i].as_ptr(), n as *mut Scalar, N);
            }
        }
        unsafe {
            std::mem::transmute_copy::<[MaybeUninit<[Scalar; N]>; M], [[Scalar; N]; M]>(&array)
        }
    }
    pub fn transpose(&self) -> Matrix<M, N> {
        let mut out = Matrix::<M, N>::default();
        for m in 0..M {
            for n in 0..N {
                out[m].push(self[n][m]);
            }
        }
        out
    }
    pub fn sum(&self) -> Scalar {
        let mut sum = 0.0;
        for m in 0..M {
            for n in 0..N {
                sum += self[m][n];
            }
        }
        sum
    }
}
