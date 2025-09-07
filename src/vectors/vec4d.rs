use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

use crate::CartesianCoords2D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct _Tensor4D<Kind> {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,

    _phantom: PhantomData<Kind>,
}

impl<Kind> _Tensor4D<Kind> {
    #[inline]
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Self {
            a,
            b,
            c,
            d,
            _phantom: PhantomData,
        }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d)
    }

    pub fn unpack(&self) -> (f64, f64, f64, f64) {
        (self.a, self.b, self.c, self.d)
    }

    pub fn unpack_as_f32(&self) -> (f32, f32, f32, f32) {
        (self.a as f32, self.b as f32, self.c as f32, self.d as f32)
    }
}

impl<Kind, T: From<f64>> Into<(T, T, T, T)> for _Tensor4D<Kind> {
    fn into(self) -> (T, T, T, T) {
        (self.a.into(), self.b.into(), self.c.into(), self.d.into())
    }
}

impl<Kind> Add for _Tensor4D<Kind> {
    type Output = _Tensor4D<Kind>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a + rhs.a,
            self.b + rhs.b,
            self.c + rhs.c,
            self.d + rhs.d,
        )
    }
}

impl<Kind> Sub for _Tensor4D<Kind> {
    type Output = _Tensor4D<Kind>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a - rhs.a,
            self.b - rhs.b,
            self.c - rhs.c,
            self.d - rhs.d,
        )
    }
}

impl<Kind> Mul for _Tensor4D<Kind> {
    type Output = _Tensor4D<Kind>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a * rhs.a,
            self.b * rhs.b,
            self.c * rhs.c,
            self.d * rhs.d,
        )
    }
}

impl<Kind, T: Into<f64>> Mul<T> for _Tensor4D<Kind> {
    type Output = _Tensor4D<Kind>;

    fn mul(self, rhs: T) -> Self::Output {
        let val: f64 = rhs.into();
        Self::new(self.a * val, self.b * val, self.c * val, self.d * val)
    }
}

impl<Kind> Div for _Tensor4D<Kind> {
    type Output = _Tensor4D<Kind>;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a / rhs.a.max(crate::DIV_EPSILON),
            self.b / rhs.b.max(crate::DIV_EPSILON),
            self.c / rhs.c.max(crate::DIV_EPSILON),
            self.d / rhs.d.max(crate::DIV_EPSILON),
        )
    }
}

impl<Kind, T: Into<f64>> Div<T> for _Tensor4D<Kind> {
    type Output = _Tensor4D<Kind>;

    fn div(self, rhs: T) -> Self::Output {
        let val: f64 = rhs.into().max(crate::DIV_EPSILON);
        Self::new(self.a / val, self.b / val, self.c / val, self.d / val)
    }
}

pub type SphericalCoords4D = _Tensor4D<super::Spherical>;
pub type CartesianCoords4D = _Tensor4D<super::Cartesian>;
pub type Tensor4D = _Tensor4D<()>;

impl SphericalCoords4D {
    #[inline]
    pub fn spherical(t: f64, r: f64, theta: f64, phi: f64) -> Self {
        Self::new(t, r, theta, phi)
    }

    pub fn t(&self) -> f64 {
        self.a
    }
    pub fn r(&self) -> f64 {
        self.b
    }
    pub fn theta(&self) -> f64 {
        self.c
    }
    pub fn phi(&self) -> f64 {
        self.d
    }
    pub fn t_mut(&mut self) -> &mut f64 {
        &mut self.a
    }
    pub fn r_mut(&mut self) -> &mut f64 {
        &mut self.b
    }
    pub fn theta_mut(&mut self) -> &mut f64 {
        &mut self.c
    }
    pub fn phi_mut(&mut self) -> &mut f64 {
        &mut self.d
    }

    pub fn to_cartesian(&self) -> CartesianCoords4D {
        let r = self.r();
        let theta = self.theta();
        let phi = self.phi();

        let r_sin_theta = r * theta.sin();
        let x = r_sin_theta * phi.cos();
        let y = r_sin_theta * phi.sin();
        let z = r * theta.cos();
        CartesianCoords4D::cartesian(self.t(), x, y, z)
    }
}

impl CartesianCoords4D {
    #[inline]
    pub fn cartesian(t: f64, x: f64, y: f64, z: f64) -> Self {
        Self::new(t, x, y, z)
    }

    pub fn t(&self) -> f64 {
        self.a
    }
    pub fn x(&self) -> f64 {
        self.b
    }
    pub fn y(&self) -> f64 {
        self.c
    }
    pub fn z(&self) -> f64 {
        self.d
    }
    pub fn t_mut(&mut self) -> &mut f64 {
        &mut self.a
    }
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.b
    }
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.c
    }
    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.d
    }

    pub fn unpack_xy(self) -> (f64, f64) {
        (self.x(), self.y())
    }
    pub fn unpack_xy_as_f32(self) -> (f32, f32) {
        (self.x() as f32, self.y() as f32)
    }

    pub fn to_spherical(&self) -> SphericalCoords4D {
        let r = self.magnitude();
        let theta = if r == 0. { 0. } else { f64::acos(self.z() / r) };
        let phi = f64::atan2(self.y(), self.x());
        SphericalCoords4D::spherical(self.t(), r, theta, phi)
    }
}

impl Add<CartesianCoords2D> for CartesianCoords4D {
    type Output = CartesianCoords4D;

    fn add(self, rhs: CartesianCoords2D) -> Self::Output {
        Self::cartesian(self.t(), self.x() + rhs.x(), self.y() + rhs.y(), self.z())
    }
}

impl From<SphericalCoords4D> for CartesianCoords4D {
    fn from(value: SphericalCoords4D) -> Self {
        value.to_cartesian()
    }
}

impl From<CartesianCoords4D> for SphericalCoords4D {
    fn from(value: CartesianCoords4D) -> Self {
        value.to_spherical()
    }
}
