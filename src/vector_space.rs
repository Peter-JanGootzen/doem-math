use staticvec::StaticVec;

use std::fmt;
use std::mem::MaybeUninit;

pub type Vector<const M: usize> = Matrix<M, 1>;
pub type Vector2 = Vector<2>;
pub type Vector3 = Vector<3>;
pub type Vector4 = Vector<4>;

pub type SquareMatrix<const M: usize> = Matrix<M, M>;
pub type Matrix2 = SquareMatrix<2>;
pub type Matrix3 = SquareMatrix<3>;
pub type Matrix4 = SquareMatrix<4>;
pub type Scalar = f32;
pub const PI: f32 = std::f32::consts::PI;


pub struct Matrix<const M: usize, const N: usize> {
    // M = rows
    // N = columns
    pub data: StaticVec<StaticVec<Scalar, N>, M>,
}

impl<const M: usize, const N: usize> Clone for Matrix<M, N> {
    fn clone(&self) -> Matrix<M, N> {
        Matrix::<M, N> {
            data: self.data.clone()
        }
    }
}

impl<const M: usize> Vector<M> {
    pub fn dot_product(&self, rhs: &Vector<M>) -> Scalar {
        let mut total = 0.0;
        for m in 0..M {
            total += self.data[m][0] * rhs.data[m][0];
        }
        total
    }
}

impl<const M: usize, const N: usize> fmt::Display for Matrix<M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_owned();
        for m in 0..M {
            s.push_str("|");
            for n in 0..N {
                s.push_str(&self.data[m][n].to_string());
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

impl<const M: usize> SquareMatrix<M> {
    pub fn identity() -> SquareMatrix<M> {
        let mut out = SquareMatrix::<M>::default();
        for (i, m) in out.data.iter_mut().enumerate() {
            for n in 0..M {
                if i == n {
                    m.push(1.0);
                } else {
                    m.push(0.0);
                }
            }
        }
        out
    }
    pub fn get_translation(translation: &Vector<{M - 1}>) -> SquareMatrix<M>  {
        let mut out = SquareMatrix::<M>::identity();
        for m in 0..M - 1 {
            out[m][M - 1] = translation[m][0];
        }
        out
    }
    pub fn get_scaling<const P: usize>(scaling: &Vector<P>) -> SquareMatrix<M>  {
        // todo static assert
        assert!(P <= M, "The size of the vector is too big for this type of matrix");
        let mut out = SquareMatrix::<M>::identity();
        for p in 0..P {
            out[p][p] = scaling[p][0]; 
        }
        out
    }
    // Rotations
    pub fn new_2d_rotation_x(angle: Scalar) -> SquareMatrix<M> {
        assert!(M > 1);
        let mut out = SquareMatrix::<M>::identity();
        out.data[1][1] = angle.cos();
        out.data[2][1] = angle.sin();
        out.data[1][2] = -angle.sin();
        out.data[2][2] = angle.cos();

        out
    }
    pub fn new_2d_rotation_y(angle: Scalar) -> SquareMatrix<M> {
        assert!(M > 1);
        let mut out = SquareMatrix::<M>::identity();
        out.data[0][0] = angle.cos();
        out.data[0][2] = angle.sin();
        out.data[2][0] = -angle.sin();
        out.data[2][2] = angle.cos();
        out
    }
    pub fn new_2d_rotation_z(angle: Scalar) -> SquareMatrix<M> {
        assert!(M > 1);
        let mut out = SquareMatrix::<M>::identity();
        out.data[0][0] = angle.cos();
        out.data[0][1] = -angle.sin();
        out.data[1][0] = angle.sin();
        out.data[1][1] = angle.cos();

        out
    }
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new_from_array(array: [[Scalar; N]; M]) -> Matrix<M, N> {
        let mut data = StaticVec::<StaticVec<Scalar, N>, M>::new();
        for n in array.iter() {
            data.push(StaticVec::<Scalar, N>::new());
            for m in n.iter() {
                data.last_mut().unwrap().push(*m);
            }
        }
        Matrix::<M, N> { data }
    }
    pub fn new_from_staticvec(staticvec: StaticVec<StaticVec<Scalar, N>, M>) -> Matrix<M, N> {
        Matrix::<M, N> { data: staticvec }
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
}

impl<const M: usize> Vector<M> {
    pub fn normalize(&self) -> Vector<M> {
        self * (1.0 / self.length())
    }
    pub fn length(&self) -> Scalar {
        let mut total = 0.0;
        for m in 0..M {
            total += self[m][0].powi(2);
        }
        total.sqrt()
    }
}
impl<const M: usize, const N: usize> std::ops::Mul<Scalar>
    for &Matrix<M, N>
{
    type Output = Matrix<M, N>;
    fn mul(self, rhs: Scalar) -> Self::Output {
        let mut out = Matrix::<M, N>::new_from_array(self.copy_to_array());

        for m in 0..M {
            for n in 0..N {
                out[m][n] = self[m][n] * rhs;
            }
        }

        out
    }
}

type Foo<const N: usize> = [i32; N + 1];

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
                    f += self.data[m][k] * rhs.data[k][p];
                }
                m_data.insert(p, f);
            }
        }
        out
    }
}

impl<const M: usize, const N: usize> std::ops::Index<usize> for Matrix<M, N>
{
    type Output = StaticVec<Scalar, N>;
    fn index<'a> (&'a self, index: usize) -> &'a Self::Output {
        &self.data[index]
    }
}

impl<const M: usize, const N: usize> std::ops::IndexMut<usize> for Matrix<M, N>
{
    fn index_mut<'a> (&'a mut self, index: usize) -> &'a mut Self::Output {
        &mut self.data[index]
    }
}
