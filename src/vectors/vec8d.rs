use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

use crate::{CartesianCoords4D, SphericalCoords4D};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct _Tensor8D<Kind> {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
    pub g: f64,
    pub h: f64,

    _phantom: PhantomData<Kind>,
}

impl<Kind> _Tensor8D<Kind> {
    #[inline]
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            _phantom: PhantomData,
        }
    }

    pub fn magnitude(self) -> f64 {
        f64::sqrt(
            self.a * self.a
                + self.b * self.b
                + self.c * self.c
                + self.d * self.d
                + self.e * self.e
                + self.f * self.f
                + self.g * self.g
                + self.h * self.h,
        )
    }

    pub fn unpack(self) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        (
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h,
        )
    }

    pub fn unpack_as_f32(self) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
        (
            self.a as f32,
            self.b as f32,
            self.c as f32,
            self.d as f32,
            self.e as f32,
            self.f as f32,
            self.g as f32,
            self.h as f32,
        )
    }
}

impl<Kind, T: From<f64>> Into<(T, T, T, T, T, T, T, T)> for _Tensor8D<Kind> {
    fn into(self) -> (T, T, T, T, T, T, T, T) {
        (
            self.a.into(),
            self.b.into(),
            self.c.into(),
            self.d.into(),
            self.e.into(),
            self.f.into(),
            self.g.into(),
            self.h.into(),
        )
    }
}

impl<Kind> Add for _Tensor8D<Kind> {
    type Output = _Tensor8D<Kind>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a + rhs.a,
            self.b + rhs.b,
            self.c + rhs.c,
            self.d + rhs.d,
            self.e + rhs.e,
            self.f + rhs.f,
            self.g + rhs.g,
            self.h + rhs.h,
        )
    }
}

impl<Kind> Sub for _Tensor8D<Kind> {
    type Output = _Tensor8D<Kind>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a - rhs.a,
            self.b - rhs.b,
            self.c - rhs.c,
            self.d - rhs.d,
            self.e - rhs.e,
            self.f - rhs.f,
            self.g - rhs.g,
            self.h - rhs.h,
        )
    }
}

impl<Kind> Mul for _Tensor8D<Kind> {
    type Output = _Tensor8D<Kind>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a * rhs.a,
            self.b * rhs.b,
            self.c * rhs.c,
            self.d * rhs.d,
            self.e * rhs.e,
            self.f * rhs.f,
            self.g * rhs.g,
            self.h * rhs.h,
        )
    }
}

impl<Kind, T: Into<f64>> Mul<T> for _Tensor8D<Kind> {
    type Output = _Tensor8D<Kind>;

    fn mul(self, rhs: T) -> Self::Output {
        let val: f64 = rhs.into();
        Self::new(
            self.a * val,
            self.b * val,
            self.c * val,
            self.d * val,
            self.e * val,
            self.f * val,
            self.g * val,
            self.h * val,
        )
    }
}

impl<Kind> Div for _Tensor8D<Kind> {
    type Output = _Tensor8D<Kind>;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a / rhs.a.max(crate::DIV_EPSILON),
            self.b / rhs.b.max(crate::DIV_EPSILON),
            self.c / rhs.c.max(crate::DIV_EPSILON),
            self.d / rhs.d.max(crate::DIV_EPSILON),
            self.e / rhs.e.max(crate::DIV_EPSILON),
            self.f / rhs.f.max(crate::DIV_EPSILON),
            self.g / rhs.g.max(crate::DIV_EPSILON),
            self.h / rhs.h.max(crate::DIV_EPSILON),
        )
    }
}

impl<Kind, T: Into<f64>> Div<T> for _Tensor8D<Kind> {
    type Output = _Tensor8D<Kind>;

    fn div(self, rhs: T) -> Self::Output {
        let val: f64 = rhs.into().max(crate::DIV_EPSILON);
        Self::new(
            self.a / val,
            self.b / val,
            self.c / val,
            self.d / val,
            self.e / val,
            self.f / val,
            self.g / val,
            self.h / val,
        )
    }
}

pub type SphericalState4D = _Tensor8D<super::Spherical>;
pub type CartesianState4D = _Tensor8D<super::Cartesian>;
pub type Tensor8D = _Tensor8D<()>;

impl SphericalState4D {
    #[inline]
    pub fn spherical(
        t: f64,
        r: f64,
        theta: f64,
        phi: f64,
        dt: f64,
        dr: f64,
        dtheta: f64,
        dphi: f64,
    ) -> Self {
        Self::new(t, r, theta, phi, dt, dr, dtheta, dphi)
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
    pub fn dt(&self) -> f64 {
        self.e
    }
    pub fn dr(&self) -> f64 {
        self.f
    }
    pub fn dtheta(&self) -> f64 {
        self.g
    }
    pub fn dphi(&self) -> f64 {
        self.h
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
    pub fn dt_mut(&mut self) -> &mut f64 {
        &mut self.e
    }
    pub fn dr_mut(&mut self) -> &mut f64 {
        &mut self.f
    }
    pub fn dtheta_mut(&mut self) -> &mut f64 {
        &mut self.g
    }
    pub fn dphi_mut(&mut self) -> &mut f64 {
        &mut self.h
    }

    pub fn position(&self) -> SphericalCoords4D {
        SphericalCoords4D::spherical(self.t(), self.r(), self.theta(), self.phi())
    }

    pub fn to_cartesian(&self) -> CartesianState4D {
        let r = self.r();
        let theta = self.theta();
        let phi = self.phi();
        let dr = self.dr();
        let dtheta = self.dtheta();
        let dphi = self.dphi();

        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();

        let sin_theta_cos_phi = sin_theta * cos_phi;
        let sin_theta_sin_phi = sin_theta * sin_phi;

        let x = r * sin_theta_cos_phi;
        let y = r * sin_theta * sin_phi;
        let z = r * cos_theta;

        let dx = dr * sin_theta_cos_phi + r * dtheta * cos_theta * cos_phi
            - r * dphi * sin_theta_sin_phi;

        let dy = dr * sin_theta_sin_phi
            + r * dtheta * cos_theta * sin_phi
            + r * dphi * sin_theta_cos_phi;

        let dz = dr * cos_theta - r * dtheta * sin_theta;

        CartesianState4D::cartesian(self.t(), x, y, z, self.dt(), dx, dy, dz)
    }
}

impl CartesianState4D {
    #[inline]
    pub fn cartesian(t: f64, x: f64, y: f64, z: f64, dt: f64, dx: f64, dy: f64, dz: f64) -> Self {
        Self::new(t, x, y, z, dt, dx, dy, dz)
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
    pub fn dt(&self) -> f64 {
        self.e
    }
    pub fn dx(&self) -> f64 {
        self.f
    }
    pub fn dy(&self) -> f64 {
        self.g
    }
    pub fn dz(&self) -> f64 {
        self.h
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
    pub fn dt_mut(&mut self) -> &mut f64 {
        &mut self.e
    }
    pub fn dx_mut(&mut self) -> &mut f64 {
        &mut self.f
    }
    pub fn dy_mut(&mut self) -> &mut f64 {
        &mut self.g
    }
    pub fn dz_mut(&mut self) -> &mut f64 {
        &mut self.h
    }

    pub fn position(&self) -> crate::CartesianCoords4D {
        CartesianCoords4D::cartesian(self.t(), self.x(), self.y(), self.z())
    }

    pub fn to_spherical(&self) -> SphericalState4D {
        let x = self.x();
        let y = self.y();
        let z = self.z();
        let dx = self.dx();
        let dy = self.dy();
        let dz = self.dz();

        let r = (x * x + y * y + z * z).sqrt();
        let theta = if r == 0.0 { 0.0 } else { (z / r).acos() };
        let phi = y.atan2(x);

        let dr = if r > crate::DIV_EPSILON {
            (x * dx + y * dy + z * dz) / r
        } else {
            0.0
        };

        let rho_sq = x * x + y * y;
        let rho = rho_sq.sqrt();

        let dtheta = if r > crate::DIV_EPSILON && rho > crate::DIV_EPSILON {
            (z * (x * dx + y * dy) - rho_sq * dz) / (r * r * rho)
        } else {
            0.0
        };

        let dphi = if rho_sq > crate::DIV_EPSILON {
            (x * dy - y * dx) / rho_sq
        } else {
            0.0
        };

        SphericalState4D::spherical(self.t(), r, theta, phi, self.dt(), dr, dtheta, dphi)
    }
}

impl From<SphericalState4D> for CartesianState4D {
    fn from(value: SphericalState4D) -> Self {
        value.to_cartesian()
    }
}

impl From<CartesianState4D> for SphericalState4D {
    fn from(value: CartesianState4D) -> Self {
        value.to_spherical()
    }
}
