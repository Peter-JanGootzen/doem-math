use doem_math::vector_space::Matrix4;
use doem_math::vector_space::Matrix3;
use doem_math::vector_space::Vector3;
use doem_math::vector_space::Vector;
use doem_math::vector_space::PI;
use float_cmp::approx_eq;

#[test]
fn m4_mul_m4() {
    let m1 = Matrix4::new_from_array([
        [8.0, 0.0, 55.0, 4.0],
        [7.0, 0.0, 31.0, 3.0],
        [1.0, 14.0, 33.0, 2.0],
        [-20.0, 81249.0, 10.0, 1.0],
    ]);
    let m2 = Matrix4::new_from_array([
        [129.0, 0.0, 55.0, 2.0],
        [7.0, 144.0, 49.0, 3.0],
        [1.0, 14.0, 9.0, 2.0],
        [-20.0, 81249.0, -842.0, 1.0],
    ]);
    let m3 = &m1 * &m2;
    println!("{}", m3.data[3][1]);
    assert_eq!(
        m3.copy_to_array(),
        [
            [1007.0, 325766.0, -2433.0, 130.0],
            [874.0, 244181.0, -1862.0, 79.0],
            [220.0, 164976.0, -646.0, 112.0],
            [566153.0, 11781245.0, 3979349.0, 243728.0],
        ]
    );
}

#[test]
fn transpose() {
    let m1 = Matrix3::new_from_array([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
        [7.0, 8.0, 9.0],
    ]);
    let m1_t = m1.transpose();
    assert_eq!(
        m1_t.copy_to_array(),
        [
            [1.0, 4.0, 7.0],
            [2.0, 5.0, 8.0],
            [3.0, 6.0, 9.0],
        ]
    );
}

#[test]
fn get_translation() {
    let v1 = Vector3::new_from_array([
        [1.0],
        [2.0],
        [3.0],
    ]);
    let translation_matrix = Matrix4::get_translation(&v1);
    assert_eq!(
        translation_matrix.copy_to_array(),
        [
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 2.0],
            [0.0, 0.0, 1.0, 3.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    );
}

#[test]
fn get_scaling() {
    let v1 = Vector3::new_from_array([
        [50.0],
        [2.0],
        [3.0],
    ]);
    let translation_matrix = Matrix4::get_scaling(&v1);
    assert_eq!(
        translation_matrix.copy_to_array(),
        [
            [50.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 3.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    );
}

#[test]
fn normalize() {
    let v1 = Vector::<2>::new_from_array([
        [4.0],
        [3.0],
    ]);
    let norm_matrix = v1.normalize();
    assert_eq!(
        norm_matrix.copy_to_array(),
        [
            [0.8],
            [0.6],
        ]
    );
}

#[test]
fn mul_f32() {
    let v1 = Vector3::new_from_array([
        [50.0],
        [2.0],
        [3.0],
    ]);
    let norm_matrix = &v1 * 0.5;
    assert_eq!(
        norm_matrix.copy_to_array(),
        [
            [25.0],
            [1.0],
            [1.5]
        ]
    );
}

//#[test]
//fn rotate_x() {
//    let v1 = Vector3::new_from_array([
//        [1.0],
//        [2.0],
//        [3.0],
//    ]);
//
//    let m1 = Matrix3::new_2d_rotation_x((90.0/180.0)*PI);
//    println!("{}", m1);
//    assert!(approx_eq!(f32, m1.data[0][0], 1.0, ulps = 7));
//    assert!(approx_eq!(f32, m1.data[0][1], 0.0, ulps = 7));
//    assert!(approx_eq!(f32, m1.data[0][2], 0.0, ulps = 7));
//
//    assert!(approx_eq!(f32, m1.data[1][0], 0.0, ulps = 7));
//    assert!(approx_eq!(f32, m1.data[1][1], -0.0, ulps = 8));
//    assert!(approx_eq!(f32, m1.data[1][2], -1.0, ulps = 8));
//
//    assert!(approx_eq!(f32, m1.data[2][0], 0.0, ulps = 7));
//    assert!(approx_eq!(f32, m1.data[2][1], 1.0, ulps = 7));
//    assert!(approx_eq!(f32, m1.data[2][2], -0.0, ulps = 8));
//}

#[test]
fn dot_product() {
    let v1 = Vector3::new_from_array([
        [1.0],
        [2.0],
        [3.0],
    ]);
    let v2 = Vector3::new_from_array([
        [2.0],
        [8.0],
        [9.0],
    ]);

    assert_eq!(v1.dot_product(&v2), 45.0);
}