mod vec3d;
mod vec6d;

#[derive(Debug, Clone, Copy)]
pub struct Spherical;
#[derive(Debug, Clone, Copy)]
pub struct Cartesian;

pub use vec3d::*;
pub use vec6d::*;
