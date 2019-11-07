use std::ops;
use generic_array::GenericArray;
use generic_array::ArrayLength;

#[derive(Debug)]
pub struct Vector<N: ArrayLength<f64>> {
    pub data: GenericArray<f64, N>
}

impl<N: ArrayLength<f64>> Default for Vector<N> {
    fn default() -> Vector<N> {
        Vector {
            data: GenericArray::<f64,N>::default()
        }
    }
}

impl<N: ArrayLength<f64>> Vector<N> {
    pub fn norm(&self) -> Vector<N> {
        self * (1.0 / self.length())
    }
    pub fn length(&self) -> f64 {
        let mut total = 0.0;
        for i in 0..self.data.len() {
            total += self.data[i].powi(2);
        }
        total.sqrt()
    }
}

impl<N: ArrayLength<f64>> ops::Add<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn add(self, other: &Vector<N>) -> Vector<N> {
        let mut out = Vector::<N>::default();
        for i in 0..self.data.len() {
            out.data[i] = self.data[i] + other.data[i];
        }
        out
    }
}
impl<N: ArrayLength<f64>> ops::Sub<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;

    fn sub(self, other: &Vector<N>) -> Vector<N> {
        let mut out = Vector::<N>::default();
        for i in 0..self.data.len() {
            out.data[i] = self.data[i] - other.data[i];
        }
        out
    }
}
impl<N: ArrayLength<f64>> ops::Mul<f64> for &Vector<N> {
    type Output = Vector<N>;

    fn mul(self, scalar: f64) -> Vector<N> {
        let mut out = Vector::<N>::default();
        for i in 0..self.data.len() {
            out.data[i] = self.data[i] * scalar;
        }
        out
    }
}
