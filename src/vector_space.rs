use staticvec::StaticVec;

use std::fmt;
use std::mem::MaybeUninit;

pub type Vector<const M: usize> = Matrix<M, 1>;
pub type Vector2 = Vector<2>;
pub type Vector3 = Vector<3>;
pub type Vector4 = Vector<4>;

pub type Matrix2 = Matrix<2, 2>;
pub type Matrix3 = Matrix<3, 3>;
pub type Matrix4 = Matrix<4, 4>;
pub type Scalar = f32;
pub const PI: f32 = std::f32::consts::PI;

pub struct Matrix<const M: usize, const N: usize> {
    // M = rows
    // N = columns
    pub data: StaticVec<StaticVec<Scalar, N>, M>,
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

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new_2d_rotation_x(angle: Scalar) -> Matrix<M, N> {
           assert!(M > 1);
           assert!(N > 1);
           let mut out = Matrix::<M, N>::identity();
           out.data[1][1] = angle.cos();
           out.data[2][1] = angle.sin();
           out.data[1][2] = -angle.sin();
           out.data[2][2] = angle.cos();

           out
    }
    pub fn new_2d_rotation_y(angle: Scalar) -> Matrix<M, N> {
        assert!(M > 1);
        assert!(N > 1);
        let mut out = Matrix::<M, N>::identity();
        out.data[0][0] = angle.cos();
        out.data[0][2] = angle.sin();
        out.data[2][0] = -angle.sin();
        out.data[2][2] = angle.cos();
        print!("{}" , out);
        out
    }
    pub fn new_2d_rotation_z(angle: Scalar) -> Matrix<M, N> {
        assert!(M > 1);
        assert!(N > 1);
        let mut out = Matrix::<M, N>::identity();
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
                out.data[m].push(self.data[n][m]);
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
                    f += self.data[m][k] * rhs.data[k][p];
                }
                m_data.insert(p, f);
            }
        }
        out
    }
}
