use crate::{Matrix4, Scalar, Vector3, Vector4};

impl Matrix4 {
    pub fn get_projection(fovy: Scalar, aspect_ratio: Scalar, near: Scalar, far: Scalar) -> Self {
        let f = f32::tan(fovy / 2.0).recip();
        Matrix4::from([
            [(f / aspect_ratio), 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (far + near) / (near - far),
                (2.0 * far * near) / (near - far),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }
    pub fn get_view(eye: &Vector3, look_at: &Vector3, up: &Vector3) -> Self {
        let d = eye - look_at;
        let r = up.cross_product(&d);
        let u = d.cross_product(&r);
        let dn = d.normalize();
        let rn = r.normalize();
        let un = u.normalize();
        let transformation = Self::from([
            [rn[0][0], rn[1][0], rn[2][0], 0.0],
            [un[0][0], un[1][0], un[2][0], 0.0],
            [dn[0][0], dn[1][0], dn[2][0], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        &transformation
            * &Self::get_translation(&Vector3::from([[-eye[0][0]], [-eye[1][0]], [-eye[2][0]]]))
    }
    // A rotation Matrix around a given and normalized(!) vector
    pub fn get_rotation(rotation_vector: &Vector4, angle: Scalar) -> Matrix4 {
        let x = rotation_vector[0][0];
        let y = rotation_vector[1][0];
        let z = rotation_vector[2][0];

        // M1
        let xz = Scalar::sqrt(x.powi(2) + z.powi(2));
        let m1 = if xz != 0.0 {
            let new_x = x / xz;
            let new_z = z / xz;
            Matrix4::from([
                [new_x, 0.0, new_z, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-new_z, 0.0, new_x, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        } else {
            Matrix4::identity()
        };

        // M2
        let xyz = Scalar::sqrt(x.powi(2) + y.powi(2) + z.powi(2));
        let m2 = if xyz != 0.0 {
            Matrix4::from([
                [xz / xyz, y / xyz, 0.0, 0.0],
                [-1.0 * (y / xyz), xz / xyz, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        } else {
            Matrix4::identity()
        };

        // M3
        let m3 = Matrix4::get_rotation_x(angle);

        // M4
        let mut m4 = m2.clone();
        m4[0][1] *= -1.0;
        m4[1][0] *= -1.0;

        // M5
        let mut m5 = m1.clone();
        m5[0][2] *= -1.0;
        m5[2][0] *= -1.0;

        &(&(&(&m5 * &m4) * &m3) * &m2) * &m1
    }
}
