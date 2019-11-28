use std::ops::Mul;

trait Vector {
    fn add(&self, rhs: &Self) -> Self;
    fn sub(&self, rhs: &Self) -> Self;
    fn mul(&self, rhs: f64) -> Self;
}

#[repr(C)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    fn add(&self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
    fn sub(&self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
    fn mul(&self, s: f64) -> Vector2 {
        Vector2 {
            x: self.x * s,
            y: self.y * s
        }
    }
}

#[repr(C)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}
#[repr(C)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}
impl Vector for Vector4 {
    fn add(&self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
    fn sub(&self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
    fn mul(&self, s: f64) -> Vector4 {
        Vector4 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s
        }
    }
}


#[repr(C)]
pub struct Matrix2 {
    pub x: Vector2,
    pub y: Vector2,
}
impl Matrix2 {
    fn add(&self, rhs: &Matrix2) -> Matrix2 {
        Matrix2 {
            x: self.x.add(&rhs.x),
            y: self.y.add(&rhs.y),
        }
    }
    fn sub(&self, rhs: &Matrix2) -> Matrix2 {
        Matrix2 {
            x: self.x.sub(&rhs.x),
            y: self.y.sub(&rhs.y),
        }
    }
}

impl Mul<&Vector2> for &Matrix2 {
    type Output = Vector2;
    fn mul(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x.x * rhs.x + self.y.x * rhs.y,
            y: self.x.y * rhs.x + self.y.y * rhs.y
        }
    }
}

#[repr(C)]
struct Matrix3 {
    x: Vector3,
    y: Vector3,
    z: Vector3
}

#[repr(C)]
pub struct Matrix4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4
}

impl Mul<&Vector4> for &Matrix4 {
    type Output = Vector4;
    fn mul(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x.x * rhs.x + self.y.x * rhs.y + self.z.x * rhs.z + self.w.x * rhs.w,
            y: self.x.y * rhs.x + self.y.y * rhs.y + self.z.y * rhs.z + self.w.y * rhs.w,
            z: self.x.z * rhs.x + self.y.z * rhs.y + self.z.z * rhs.z + self.w.z * rhs.w,
            w: self.x.w * rhs.x + self.y.w * rhs.y + self.z.w * rhs.z + self.w.w * rhs.w
        }
    }
}
impl Mul<&Matrix4> for &Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: &Matrix4) -> Matrix4 {
        Matrix4 {
            x: Vector4 {
                x: self.x.x * rhs.x.x + self.y.x * rhs.x.y + self.z.x * rhs.x.z + self.w.x * rhs.x.w,
                y: self.x.x * rhs.y.x + self.y.x * rhs.y.y + self.z.x * rhs.y.z + self.w.x * rhs.y.w,
                z: self.x.x * rhs.z.x + self.y.x * rhs.z.y + self.z.x * rhs.z.z + self.w.x * rhs.z.w,
                w: self.x.x * rhs.w.x + self.y.x * rhs.w.y + self.z.x * rhs.w.z + self.w.x * rhs.w.w
            },
            y: Vector4 {
                x: self.x.y * rhs.x.x + self.y.y * rhs.x.y + self.z.y * rhs.x.z + self.w.y * rhs.x.w,
                y: self.x.y * rhs.y.x + self.y.y * rhs.y.y + self.z.y * rhs.y.z + self.w.y * rhs.y.w,
                z: self.x.y * rhs.z.x + self.y.y * rhs.z.y + self.z.y * rhs.z.z + self.w.y * rhs.z.w,
                w: self.x.y * rhs.w.x + self.y.y * rhs.w.y + self.z.y * rhs.w.z + self.w.y * rhs.w.w
            },
            z: Vector4 {
                x: self.x.z * rhs.x.x + self.y.z * rhs.x.y + self.z.z * rhs.x.z + self.w.z * rhs.x.w,
                y: self.x.z * rhs.y.x + self.y.z * rhs.y.y + self.z.z * rhs.y.z + self.w.z * rhs.y.w,
                z: self.x.z * rhs.z.x + self.y.z * rhs.z.y + self.z.z * rhs.z.z + self.w.z * rhs.z.w,
                w: self.x.z * rhs.w.x + self.y.z * rhs.w.y + self.z.z * rhs.w.z + self.w.z * rhs.w.w
            },
            w: Vector4 {
                x: self.x.w * rhs.x.x + self.y.w * rhs.x.y + self.z.w * rhs.x.z + self.w.w * rhs.x.w,
                y: self.x.w * rhs.y.x + self.y.w * rhs.y.y + self.z.w * rhs.y.z + self.w.w * rhs.y.w,
                z: self.x.w * rhs.z.x + self.y.w * rhs.z.y + self.z.w * rhs.z.z + self.w.w * rhs.z.w,
                w: self.x.w * rhs.w.x + self.y.w * rhs.w.y + self.z.w * rhs.w.z + self.w.w * rhs.w.w
            }
        }
    }
}
