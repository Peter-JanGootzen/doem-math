use crate::Scalar;
use crate::Vector;
use crate::Vector3;

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
    pub fn dimension_hop<const P: usize>(&self) -> Vector<P> {
        let mut out = Vector::<P>::default();
        for (p, p_data) in out.data.iter_mut().enumerate() {
            if p < M {
                p_data.insert(0, self[p][0]);
            } else {
                p_data.insert(0, 1.0);
            }
        }
        out
    }
    pub fn sign_length(&self) -> Scalar {
        self.length() * self.sum().signum()
    }
    pub fn dot_product(&self, rhs: &Vector<M>) -> Scalar {
        let mut total = 0.0;
        for m in 0..M {
            total += self[m][0] * rhs[m][0];
        }
        total
    }
}

// Taking the short cut here and only implementing for Vector3
impl Vector3 {
    pub fn cross_product(&self, rhs: &Vector3) -> Self {
        Self::from([
            [self[1][0] * rhs[2][0] - self[2][0] * rhs[1][0]],
            [self[2][0] * rhs[0][0] - self[0][0] * rhs[2][0]],
            [self[0][0] * rhs[1][0] - self[1][0] * rhs[0][0]],
        ])
    }
}
