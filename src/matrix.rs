use generic_array::GenericArray;
use generic_array::ArrayLength;

#[derive(Debug)]
pub struct Matrix<M: ArrayLength<GenericArray<f64, N>>, N: ArrayLength<f64>> {
    pub data: GenericArray<GenericArray<f64, N>, M>
}

impl<M: ArrayLength<GenericArray<f64, N>>, N: ArrayLength<f64>> Default for Matrix<M, N> {
    fn default() -> Matrix<M, N> {
        Matrix {
            data: GenericArray::<GenericArray<f64, N>, M>::default()
        }
    }
}
