mod vector;
use vector::Vector;
use generic_array::typenum::{ U2, U3 };

fn main() {
    let mut v1 = Vector::<U3>::default();
    v1.data[0] = 1.0;
    v1.data[1] = 2.0;
    v1.data[2] = 3.0;
    let mut v2 = Vector::<U3>::default();
    v2.data[0] = 1.0;
    v2.data[1] = 2.0;
    v2.data[2] = 3.0;

    println!("{:?}", &v1 + &v2);

    let mut v3 = Vector::<U2>::default();
    v3.data[0] = 4.0;
    v3.data[1] = 3.0;
    println!("{:?}", v3.norm());
}
