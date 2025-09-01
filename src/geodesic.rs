use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
struct State {
    r: f64,
    dr: f64,
    phi: f64,
    dphi: f64,
}

impl Add for State {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            dr: self.dr + other.dr,
            phi: self.phi + other.phi,
            dphi: self.dphi + other.dphi,
        }
    }
}

impl Mul<f64> for State {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            dr: self.dr * rhs,
            phi: self.phi * rhs,
            dphi: self.dphi * rhs,
        }
    }
}

fn geodesic(state: &State, rs: f64) -> State {
    let d2phi = -2.0 * state.dr * state.dphi / state.r;
    let d2r = state.r * state.dphi * state.dphi * (1.0 - 1.5 * rs / state.r)
        - crate::SQUARED_SPEED_OF_LIGHT * rs / (2. * state.r * state.r);

    State {
        r: state.dr,
        dr: d2r,
        phi: state.dphi,
        dphi: d2phi,
    }
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

pub fn solve_geodesic(
    r: f64,
    dr: f64,
    phi: f64,
    dphi: f64,
    rs: f64,
    h: f64,
) -> (f64, f64, f64, f64) {
    let initial_state = State { r, dr, phi, dphi };
    let f = |state| geodesic(&state, rs);
    let final_state = runge_kutta_4(initial_state, h, f);
    (
        final_state.r,
        final_state.dr,
        final_state.phi,
        final_state.dphi,
    )
}
