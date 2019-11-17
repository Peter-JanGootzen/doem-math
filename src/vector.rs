use std::ops;
use generic_array::GenericArray;
use generic_array::ArrayLength;
use generic_array::typenum::{ U1, U2, U3, U4 };
use crate::matrix::Matrix;

pub type VectorM<M> = Matrix<M, U1>;
pub type Vector2 = VectorM<U2>;
pub type Vector3 = VectorM<U3>;
pub type Vector4 = VectorM<U4>;

//impl<N: ArrayLength<f64>> Default for VectorN<N> {
//    fn default() -> VectorN<N> {
//        VectorN {
//            data: GenericArray::<ArrayLength<f64>,N>::default()
//        }
//    }
//}

//impl<N: ArrayLength<f64>> VectorN<N> {
//    pub fn norm(&self) -> VectorN<N> {
//        self * (1.0 / self.length())
//    }
//    pub fn length(&self) -> f64 {
//        let mut total = 0.0;
//        for i in 0..self.data.len() {
//            total += self.data[i].powi(2);
//        }
//        total.sqrt()
//    }
//}

impl<M: ArrayLength<GenericArray<f64, U1>>> ops::Add<&VectorM<M>> for &VectorM<M> {
    type Output = VectorM<M>;

    fn add(self, other: &VectorM<M>) -> VectorM<M> {
        let mut out = VectorM::<M>::default();
        for row in 0..self.data.len() {
            out.data[row][0] = self.data[row][0] + other.data[row][0];
        }
        out
    }
}
//impl<N: ArrayLength<f64>> ops::Sub<&Vector<N>> for &Vector<N> {
//    type Output = Vector<N>;
//
//    fn sub(self, other: &Vector<N>) -> Vector<N> {
//        let mut out = Vector::<N>::default();
//        for i in 0..self.data.len() {
//            out.data[i] = self.data[i] - other.data[i];
//        }
//        out
//    }
//}
//impl<N: ArrayLength<f64>> ops::Mul<f64> for &Vector<N> {
//    type Output = Vector<N>;
//
//    fn mul(self, scalar: f64) -> Vector<N> {
//        let mut out = Vector::<N>::default();
//        for i in 0..self.data.len() {
//            out.data[i] = self.data[i] * scalar;
//        }
//        out
//    }
//}

