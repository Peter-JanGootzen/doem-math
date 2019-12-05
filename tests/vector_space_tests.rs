use rusty_linear_algebra::vector_space::Matrix4;

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
        m3.into_array(),
        [
            [1007.0, 325766.0, -2433.0, 130.0],
            [874.0, 244181.0, -1862.0, 79.0],
            [220.0, 164976.0, -646.0, 112.0],
            [566153.0, 11781245.0, 3979349.0, 243728.0],
        ]
    );
}
