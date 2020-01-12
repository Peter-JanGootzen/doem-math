#![feature(const_generics)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice_assume_init)]
#![allow(dead_code)]
#![allow(incomplete_features)]

mod matrix;
mod matrix4;
mod operators;
mod square_matrix;
mod trait_impls;
mod vector;

use staticvec::StaticVec;

pub type Scalar = f32;
pub const PI: f32 = std::f32::consts::PI;

#[derive(Clone)]
pub struct Matrix<const M: usize, const N: usize> {
    // M = rows
    // N = columns
    pub data: StaticVec<StaticVec<Scalar, N>, M>,
}

pub type Vector<const M: usize> = Matrix<M, 1>;
pub type Vector2 = Vector<2>;
pub type Vector3 = Vector<3>;
pub type Vector4 = Vector<4>;

pub type SquareMatrix<const M: usize> = Matrix<M, M>;
pub type Matrix2 = SquareMatrix<2>;
pub type Matrix3 = SquareMatrix<3>;
pub type Matrix4 = SquareMatrix<4>;
