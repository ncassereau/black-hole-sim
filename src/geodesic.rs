use std::ops::{Add, Mul};

use crate::SphericalState4D;

fn geodesic(state: SphericalState4D, rs: f64) -> SphericalState4D {
    let (sin_theta, cos_theta) = state.theta().sin_cos();
    let altitude = (state.r() - rs).max(crate::DIV_EPSILON);

    let d2t = -rs * state.dr() * state.dt() / (state.r() * altitude);

    let d2r = {
        let term1 = rs * altitude * state.dt().powi(2) / (2.0 * state.r().powi(3));
        let term2 = rs / (2.0 * state.r() * altitude) * state.dr().powi(2);
        let term3 = state.dtheta().powi(2) + (sin_theta * state.dphi()).powi(2);
        term1 - term2 - altitude * term3
    };

    let d2theta = {
        let term1 = -2.0 * state.dr() * state.dtheta() / state.r();
        let term2 = sin_theta * cos_theta * state.dphi().powi(2);
        term1 + term2
    };

    let d2phi = {
        let term1 = state.dr() * state.dphi() / state.r();
        let term2 = state.dtheta() * state.dphi() * cos_theta / sin_theta;
        -2.0 * (term1 + term2)
    };

    SphericalState4D::spherical(
        state.dt(),
        state.dr(),
        state.dtheta(),
        state.dphi(),
        d2t,
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

pub fn solve_geodesic(initial_state: SphericalState4D, rs: f64, h: f64) -> SphericalState4D {
    let f = |state| geodesic(state, rs);
    runge_kutta_4(initial_state, h, f)
}
