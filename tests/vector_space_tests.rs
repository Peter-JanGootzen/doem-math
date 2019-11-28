use rusty_linear_algebra::vector_space::Matrix4;
use rusty_linear_algebra::vector_space::Vector4;

#[test]
fn m44_mul_m44() {
    let m1 = Matrix4 {
        x: Vector4 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0
        },
        y: Vector4 {
            x: 5.0,
            y: 6.0,
            z: 7.0,
            w: 8.0
        },
        z: Vector4 {
            x: 9.0,
            y: 10.0,
            z: 11.0,
            w: 12.0
        },
        w: Vector4 {
            x: 13.0,
            y: 14.0,
            z: 15.0,
            w: 16.0
        }
    };
    let m2 = Matrix4 {
        x: Vector4 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0
        },
        y: Vector4 {
            x: 5.0,
            y: 6.0,
            z: 7.0,
            w: 8.0
        },
        z: Vector4 {
            x: 9.0,
            y: 10.0,
            z: 11.0,
            w: 12.0
        },
        w: Vector4 {
            x: 13.0,
            y: 14.0,
            z: 15.0,
            w: 16.0
        }
    };
    let m3 = &m1 * &m2;
    assert_eq!(m3.x.x, 90.0);
    assert_eq!(m3.y.x, 100.0);
    assert_eq!(m3.z.x, 110.0);
    assert_eq!(m3.w.x, 120.0);

    assert_eq!(m3.x.y, 202.0);
    assert_eq!(m3.y.y, 228.0);
    assert_eq!(m3.z.y, 254.0);
    assert_eq!(m3.w.y, 280.0);

    assert_eq!(m3.x.z, 314.0);
    assert_eq!(m3.y.z, 356.0);
    assert_eq!(m3.z.z, 398.0);
    assert_eq!(m3.w.z, 440.0);

    assert_eq!(m3.x.w, 426.0);
    assert_eq!(m3.y.w, 484.0);
    assert_eq!(m3.z.w, 542.0);
    assert_eq!(m3.w.w, 600.0);
}
