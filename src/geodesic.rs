use std::ops::{Add, Mul, Sub};

use crate::{SphericalState4D, tensors};

fn geodesic(state: SphericalState4D, rs: f64) -> SphericalState4D {
    let (sin_theta, cos_theta) = state.theta().sin_cos();
    let altitude = (state.r() - rs).max(crate::DIV_EPSILON);

    let d2t = -rs * state.dr() * state.dt() / (state.r() * altitude);

    let d2r = {
        let term1 = rs * altitude * state.dt().powi(2) / (2.0 * state.r().powi(3));
        let term2 = rs / (2.0 * state.r() * altitude) * state.dr().powi(2);
        let term3 = state.dtheta().powi(2) + (sin_theta * state.dphi()).powi(2);
        -term1 + term2 + altitude * term3
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

fn runge_kutta_fehlberg_45_pseudo_step<T, F>(state: T, h: f64, tol: f64, f: F) -> (T, f64, f64)
where
    F: Fn(T) -> T,
    T: Add<Output = T> + Sub<T, Output = T> + Mul<f64, Output = T> + tensors::Norm,
{
    let k1 = f(state);
    let k2 = f(state + k1 * (h * (1.0 / 4.0)));
    let k3 = f(state + k1 * (h * (3.0 / 32.0)) + k2 * (h * (9.0 / 32.0)));
    let k4 = f(
        state + k1 * (h * (1932.0 / 2197.0)) - k2 * (h * (7200.0 / 2197.0))
            + k3 * (h * (7296.0 / 2197.0)),
    );
    let k5 = f(
        state + k1 * (h * (439.0 / 216.0)) - k2 * (h * 8.0) + k3 * (h * (3680.0 / 513.0))
            - k4 * (h * (845.0 / 4104.0)),
    );
    let k6 = f(
        state - k1 * (h * (8.0 / 27.0)) + k2 * (h * 2.0) - k3 * (h * (3544.0 / 2565.0))
            + k4 * (h * (1859.0 / 4104.0))
            - k5 * (h * (11.0 / 40.0)),
    );

    let state_order_4 = state
        + k1 * (h * (25.0 / 216.0))
        + k3 * (h * (1408.0 / 2565.0))
        + k4 * (h * (2197.0 / 4104.0))
        - k5 * (h * (1.0 / 5.0));

    let state_order_5 = state
        + k1 * (h * (16.0 / 135.0))
        + k3 * (h * (6656.0 / 12825.0))
        + k4 * (h * (28561.0 / 56430.0))
        - k5 * (h * (9.0 / 50.0))
        + k6 * (h * (2.0 / 55.0));

    let error = (state_order_4 - state_order_5).norm();

    let new_h = 0.9 * h * (tol / error).powf(0.2);

    (state_order_5, error, new_h)
}

fn runge_kutta_fehlberg_45<T, F>(
    initial_state: T,
    h: f64,
    f: F,
    tol: f64,
    min_h: f64,
    max_retries: usize,
) -> (T, f64, bool)
where
    F: Fn(T) -> T,
    T: Add<Output = T> + Sub<T, Output = T> + Mul<f64, Output = T> + tensors::Norm + Copy,
{
    let mut current_h = h;
    let mut counter = 0;

    loop {
        let (new_state, error, mut new_h) =
            runge_kutta_fehlberg_45_pseudo_step(initial_state, current_h, tol, &f);

        if error < tol {
            if new_h > current_h * 2.0 {
                new_h = current_h * 2.0;
            }
            return (new_state, new_h, true);
        }

        counter += 1;
        if new_h < min_h || counter > max_retries {
            return (initial_state, current_h, false);
        }

        current_h = new_h;
    }
}

pub fn solve_geodesic_rkf45(
    initial_state: SphericalState4D,
    rs: f64,
    h: f64,
) -> (SphericalState4D, f64, bool) {
    let f = |state| geodesic(state, rs);

    runge_kutta_fehlberg_45(
        initial_state,
        h,
        f,
        rs * crate::RKF45_TOLERANCE_FACTOR,
        rs * crate::RKF45_MIN_STEP_FACTOR,
        crate::RFK45_RETRIES,
    )
}
