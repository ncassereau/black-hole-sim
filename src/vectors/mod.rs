mod vec2d;
mod vec3d;
mod vec4d;
mod vec6d;
mod vec8d;

#[derive(Debug, Clone, Copy)]
pub struct Spherical;
#[derive(Debug, Clone, Copy)]
pub struct Cartesian;

pub use vec2d::*;
pub use vec3d::*;
pub use vec4d::*;
pub use vec6d::*;
pub use vec8d::*;
