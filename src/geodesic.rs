use std::ops::{Add, Mul};

use crate::SphericalState3D;

fn geodesic(state: SphericalState3D, rs: f64) -> SphericalState3D {
    let d2r = state.r() * state.dphi() * state.dphi() * (1.0 - 1.5 * rs / state.r())
        - crate::SQUARED_SPEED_OF_LIGHT * rs / (2. * state.r() * state.r());
    let d2theta = 0.;
    let d2phi = -2.0 * state.dr() * state.dphi() / state.r();

    SphericalState3D::spherical(
        state.dr(),
        state.dtheta(),
        state.dphi(),
        d2r,
        d2theta,
        d2phi,
    )
}

fn runge_kutta_4<T, F>(state: T, h: f64, f: F) -> T
where
    F: Fn(T) -> T,
    T: Add<Output = T> + Mul<f64, Output = T> + Copy,
{
    let k1 = f(state);
    let k2 = f(state + k1 * (h / 2.0));
    let k3 = f(state + k2 * (h / 2.0));
    let k4 = f(state + k3 * h);

    state + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (h / 6.0)
}

pub fn solve_geodesic(initial_state: SphericalState3D, rs: f64, h: f64) -> SphericalState3D {
    let f = |state| geodesic(state, rs);
    runge_kutta_4(initial_state, h, f)
}
