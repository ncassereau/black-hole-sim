use std::ops::Div;

mod tensor2d;
mod tensor3d;
mod tensor4d;
mod tensor6d;
mod tensor8d;

#[derive(Debug, Clone, Copy)]
pub struct Spherical;
#[derive(Debug, Clone, Copy)]
pub struct Cartesian;

pub use tensor2d::*;
pub use tensor3d::*;
pub use tensor4d::*;
pub use tensor6d::*;
pub use tensor8d::*;

pub trait Norm: Div<f64, Output = Self> + Copy {
    fn norm(&self) -> f64;

    fn normalize(self) -> Self {
        self / self.norm()
    }
}
