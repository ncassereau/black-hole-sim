use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct _Tensor2D<Kind> {
    pub a: f64,
    pub b: f64,

    _phantom: PhantomData<Kind>,
}

impl<Kind: Copy> _Tensor2D<Kind> {
    #[inline]
    pub fn new(a: f64, b: f64) -> Self {
        Self {
            a,
            b,
            _phantom: PhantomData,
        }
    }

    pub fn unpack(&self) -> (f64, f64) {
        (self.a, self.b)
    }

    pub fn unpack_as_f32(&self) -> (f32, f32) {
        (self.a as f32, self.b as f32)
    }
}

impl<Kind: Copy, T: From<f64>> Into<(T, T)> for _Tensor2D<Kind> {
    fn into(self) -> (T, T) {
        (self.a.into(), self.b.into())
    }
}

impl<Kind: Copy> Add for _Tensor2D<Kind> {
    type Output = _Tensor2D<Kind>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.a + rhs.a, self.b + rhs.b)
    }
}

impl<Kind: Copy> Sub for _Tensor2D<Kind> {
    type Output = _Tensor2D<Kind>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.a - rhs.a, self.b - rhs.b)
    }
}

impl<Kind: Copy> Mul for _Tensor2D<Kind> {
    type Output = _Tensor2D<Kind>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.a * rhs.a, self.b * rhs.b)
    }
}

impl<Kind: Copy, T: Into<f64>> Mul<T> for _Tensor2D<Kind> {
    type Output = _Tensor2D<Kind>;

    fn mul(self, rhs: T) -> Self::Output {
        let val: f64 = rhs.into();
        Self::new(self.a * val, self.b * val)
    }
}

impl<Kind: Copy> Div for _Tensor2D<Kind> {
    type Output = _Tensor2D<Kind>;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a / rhs.a.max(crate::DIV_EPSILON),
            self.b / rhs.b.max(crate::DIV_EPSILON),
        )
    }
}

impl<Kind: Copy, T: Into<f64>> Div<T> for _Tensor2D<Kind> {
    type Output = _Tensor2D<Kind>;

    fn div(self, rhs: T) -> Self::Output {
        let val: f64 = rhs.into().max(crate::DIV_EPSILON);
        Self::new(self.a / val, self.b / val)
    }
}

pub type CartesianCoords2D = _Tensor2D<super::Cartesian>;
pub type Tensor2D = _Tensor2D<()>;

impl CartesianCoords2D {
    #[inline]
    pub fn cartesian(x: f64, y: f64) -> Self {
        Self::new(x, y)
    }

    pub fn x(&self) -> f64 {
        self.a
    }
    pub fn y(&self) -> f64 {
        self.b
    }
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.a
    }
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.b
    }
}
