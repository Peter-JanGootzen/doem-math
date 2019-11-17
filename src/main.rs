mod vector;
mod matrix;
use matrix::Matrix;
use vector::Vector2;
use generic_array::typenum::{ U1, U2, U3 };

fn main() {
    let mut v1 = Matrix::<U3, U1>::default();
    let mut v2 = Vector2::default();
    v2.data[0][0] = 0.0;
    v2.data[1][0] = 1.0;
    let mut v3 = Vector2::default();
    v2.data[0][0] = 1.0;
    v2.data[1][0] = 2.0;

    println!("{:?}", &v2 + &v3);
}
