use staticvec::StaticVec;

use std::mem::MaybeUninit;

pub type Vector<const M: usize> = Matrix<M, 1>;
pub type Vector2 = Matrix<2, 1>;
pub type Vector3 = Matrix<3, 1>;
pub type Vector4 = Matrix<4, 1>;

pub type Matrix2 = Matrix<2, 2>;
pub type Matrix3 = Matrix<3, 3>;
pub type Matrix4 = Matrix<4, 4>;
pub type Scalar = f32;

pub struct Matrix<const M: usize, const N: usize> {
    // M = rows
    // N = columns
    pub data: StaticVec::<StaticVec::<Scalar, N>, M>
}

impl<const M: usize, const N: usize> Default for Matrix<M, N> {
    fn default() -> Self {
        let mut data = StaticVec::<StaticVec::<Scalar, N>, M>::new();
        for m in 0..M {
            data.push(StaticVec::<Scalar, N>::new());
        }
        Self {
            data: data
        }
    }
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new_from_array(array: [[Scalar; N]; M]) -> Matrix<M, N> {
        let mut data = StaticVec::<StaticVec::<Scalar, N>, M>::new();
        for n in array.iter() {
            data.push(StaticVec::<Scalar, N>::new());
            for m in n.iter() {
                data.last_mut().unwrap().push(*m);
            }
        }
        Matrix::<M, N> {
            data: data
        }
    }
    pub fn new_from_staticvec(staticvec: StaticVec::<StaticVec::<Scalar, N>, M>) -> Matrix<M, N> {
        Matrix::<M, N> {
            data: staticvec
        }
    }
    pub fn identity() -> Matrix<M, N> {
        let mut out = Matrix::<M, N>::default();
        for (i, m) in out.data.iter_mut().enumerate() {
            for n in 0..N {
                if i == n {
                    m.push(1.0);
                } else {
                    m.push(0.0);
                }
            }
        }
        out
    }
    pub fn into_array(&self) -> [[Scalar; N]; M] {
        let mut array: [MaybeUninit::<[Scalar; N]>; M] = {
            MaybeUninit::uninit_array()
        };
        // For some reason, this code only works if I println M
        println!("M = {}", M);
        for i in 0..M {
            unsafe {
                let n = array.get_unchecked_mut(i).as_mut_ptr();
                std::ptr::copy_nonoverlapping(self.data[i].as_ptr(), n as *mut Scalar, std::mem::size_of::<[Scalar; N]>());
            }
        }
        unsafe {
            std::mem::transmute_copy::<[MaybeUninit::<[Scalar; N]>; M], [[Scalar; N]; M]>(&array)
        }
    }
}

// Matrix<M, N> * Matrix<N, P> = Matrix<M, P>
impl<const M: usize, const N: usize, const P: usize> std::ops::Mul<&Matrix<N, P>> for &Matrix<M, N> {
    type Output = Matrix<M, P>;
    fn mul(self, rhs: &Matrix<N, P>) -> Self::Output {
        let mut out = Matrix::<M, P>::default();
        // Do matrix multiplication
        for (m, m_data) in out.data.iter_mut().enumerate() {
            for p in 0..P {
                let mut f = 0.0;
                for k in 0..M {
                    f +=  self.data[m][k] * rhs.data[k][p];
                    println!("{} * {} = {}", rhs.data[m][k], self.data[k][p], rhs.data[m][k] * self.data[k][p]);
                }
                m_data.insert(p, f);
            }
        }
        out
    }
}