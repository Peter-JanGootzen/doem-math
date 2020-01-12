use crate::Scalar;
use crate::SquareMatrix;
use crate::Vector;

impl<const M: usize> SquareMatrix<M> {
    pub fn identity() -> Self {
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
    pub fn get_translation(translation: &Vector<{ M - 1 }>) -> Self {
        let mut out = SquareMatrix::<M>::identity();
        for m in 0..M - 1 {
            out[m][M - 1] = translation[m][0];
        }
        out
    }
    pub fn get_scaling<const P: usize>(scaling: &Vector<P>) -> Self {
        // todo static assert
        assert!(
            P <= M,
            "The size of the vector is too big for this type of matrix"
        );
        let mut out = SquareMatrix::<M>::identity();
        for p in 0..P {
            out[p][p] = scaling[p][0];
        }
        out
    }
    // Rotations
    pub fn get_rotation_x(angle: Scalar) -> Self {
        assert!(M > 1);

        let mut out = SquareMatrix::<M>::identity();
        out[1][1] = angle.cos();
        out[2][1] = angle.sin();
        out[1][2] = -angle.sin();
        out[2][2] = angle.cos();
        out
    }
    pub fn get_rotation_y(angle: Scalar) -> Self {
        assert!(M > 1);

        let mut out = SquareMatrix::<M>::identity();
        out[0][0] = angle.cos();
        out[0][2] = angle.sin();
        out[2][0] = -angle.sin();
        out[2][2] = angle.cos();
        out
    }
    pub fn get_rotation_z(angle: Scalar) -> Self {
        assert!(M > 1);

        let mut out = SquareMatrix::<M>::identity();
        out[0][0] = angle.cos();
        out[0][1] = -angle.sin();
        out[1][0] = angle.sin();
        out[1][1] = angle.cos();
        out
    }
}
