use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct _Tensor3D<Kind> {
    pub a: f64,
    pub b: f64,
    pub c: f64,

    _phantom: PhantomData<Kind>,
}

impl<Kind> _Tensor3D<Kind> {
    #[inline]
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self {
            a,
            b,
            c,
            _phantom: PhantomData,
        }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.a * self.a + self.b * self.b + self.c * self.c)
    }

    pub fn unpack(&self) -> (f64, f64, f64) {
        (self.a, self.b, self.c)
    }

    pub fn unpack_as_f32(&self) -> (f32, f32, f32) {
        (self.a as f32, self.b as f32, self.c as f32)
    }
}

impl<Kind, T: From<f64>> Into<(T, T, T)> for _Tensor3D<Kind> {
    fn into(self) -> (T, T, T) {
        (self.a.into(), self.b.into(), self.c.into())
    }
}

impl<Kind> Add for _Tensor3D<Kind> {
    type Output = _Tensor3D<Kind>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.a + rhs.a, self.b + rhs.b, self.c + rhs.c)
    }
}

impl<Kind> Sub for _Tensor3D<Kind> {
    type Output = _Tensor3D<Kind>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.a - rhs.a, self.b - rhs.b, self.c - rhs.c)
    }
}

impl<Kind> Mul for _Tensor3D<Kind> {
    type Output = _Tensor3D<Kind>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.a * rhs.a, self.b * rhs.b, self.c * rhs.c)
    }
}

impl<Kind, T: Into<f64>> Mul<T> for _Tensor3D<Kind> {
    type Output = _Tensor3D<Kind>;

    fn mul(self, rhs: T) -> Self::Output {
        let val = rhs.into();
        Self::new(self.a * val, self.b * val, self.c * val)
    }
}

impl<Kind> Div for _Tensor3D<Kind> {
    type Output = _Tensor3D<Kind>;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a / rhs.a.max(crate::DIV_EPSILON),
            self.b / rhs.b.max(crate::DIV_EPSILON),
            self.c / rhs.c.max(crate::DIV_EPSILON),
        )
    }
}

impl<Kind, T: Into<f64>> Div<T> for _Tensor3D<Kind> {
    type Output = _Tensor3D<Kind>;

    fn div(self, rhs: T) -> Self::Output {
        let val = rhs.into().max(crate::DIV_EPSILON);
        Self::new(self.a / val, self.b / val, self.c / val)
    }
}

pub type SphericalCoords3D = _Tensor3D<super::Spherical>;
pub type CartesianCoords3D = _Tensor3D<super::Cartesian>;
pub type Tensor3D = _Tensor3D<()>;

impl SphericalCoords3D {
    #[inline]
    pub fn spherical(r: f64, theta: f64, phi: f64) -> Self {
        Self::new(r, theta, phi)
    }

    pub fn r(&self) -> f64 {
        self.a
    }
    pub fn theta(&self) -> f64 {
        self.b
    }
    pub fn phi(&self) -> f64 {
        self.c
    }
    pub fn r_mut(&mut self) -> &mut f64 {
        &mut self.a
    }
    pub fn theta_mut(&mut self) -> &mut f64 {
        &mut self.b
    }
    pub fn phi_mut(&mut self) -> &mut f64 {
        &mut self.c
    }

    pub fn to_cartesian(&self) -> CartesianCoords3D {
        let r = self.r();
        let theta = self.theta();
        let phi = self.phi();

        let r_sin_theta = r * theta.sin();
        let x = r_sin_theta * phi.cos();
        let y = r_sin_theta * phi.sin();
        let z = r * theta.cos();
        CartesianCoords3D::cartesian(x, y, z)
    }
}

impl CartesianCoords3D {
    #[inline]
    pub fn cartesian(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.a
    }
    pub fn y(&self) -> f64 {
        self.b
    }
    pub fn z(&self) -> f64 {
        self.c
    }
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.a
    }
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.b
    }
    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.c
    }

    pub fn to_spherical(&self) -> SphericalCoords3D {
        let r = self.magnitude();
        let theta = if r == 0. { 0. } else { f64::acos(self.z() / r) };
        let phi = f64::atan2(self.y(), self.x());
        SphericalCoords3D::spherical(r, theta, phi)
    }
}

impl From<SphericalCoords3D> for CartesianCoords3D {
    fn from(value: SphericalCoords3D) -> Self {
        value.to_cartesian()
    }
}

impl From<CartesianCoords3D> for SphericalCoords3D {
    fn from(value: CartesianCoords3D) -> Self {
        value.to_spherical()
    }
}
