use crate::{BlackHole, Skybox};
use crate::{CartesianCoords3D, CartesianState3D, SphericalState4D};
use macroquad::prelude::*;
use std::sync::Arc;

pub enum StoppingCriterion {
    EnteredEventHorizon,
    OutOfBoundingBox(CartesianCoords3D),
    CrossedAccretionDisk(f64),
}

fn determine_color(
    stopping_criterion: &StoppingCriterion,
    black_hole: BlackHole,
    skybox: &Arc<Skybox>,
) -> Color {
    match stopping_criterion {
        StoppingCriterion::EnteredEventHorizon => black_hole.color(),
        StoppingCriterion::OutOfBoundingBox(direction) => skybox.sample(direction),
        StoppingCriterion::CrossedAccretionDisk(r) => black_hole
            .accretion_disk()
            .get_color(*r)
            .unwrap_or(Color::from_rgba(0, 0, 0, 0)),
    }
}

fn blend(accumulated_color: Color, sample_color: Color, transmittance: f32) -> (Color, f32) {
    let new_transmittance = transmittance * (1.0 - sample_color.a);

    let r = accumulated_color.r + sample_color.r * sample_color.a * transmittance;
    let g = accumulated_color.g + sample_color.g * sample_color.a * transmittance;
    let b = accumulated_color.b + sample_color.b * sample_color.a * transmittance;

    (
        Color::new(r, g, b, 1.0 - new_transmittance),
        new_transmittance,
    )
}

fn gamma_correct(linear_color: Color) -> Color {
    const INVERSE_GAMMA: f32 = 1.0 / 2.2;
    Color {
        r: linear_color.r.powf(INVERSE_GAMMA),
        g: linear_color.g.powf(INVERSE_GAMMA),
        b: linear_color.b.powf(INVERSE_GAMMA),
        a: linear_color.a,
    }
}

pub struct Ray {
    state: SphericalState4D,
    dλ: f64,
}

impl Ray {
    pub fn new(spatial_state: CartesianState3D, rs: f64, dλ0: f64) -> Self {
        Self {
            state: spatial_state.to_spherical().to_4d(rs),
            dλ: dλ0,
        }
    }

    pub fn step(
        &mut self,
        black_hole: BlackHole,
        bounding_box_radius: f64,
    ) -> Option<StoppingCriterion> {
        if self.state.r() <= black_hole.visual_radius() {
            return Some(StoppingCriterion::EnteredEventHorizon);
        }

        let (state, dλ) =
            match crate::geodesic::solve_geodesic_rkf45(self.state, black_hole.radius(), self.dλ) {
                Ok((state, dλ)) => (state, dλ),
                Err(_) => {
                    if self.state.r() < black_hole.visual_radius() {
                        // RKF Step failed because we are very close to Black Hole. We therefore consider that we fell into it.
                        return Some(StoppingCriterion::EnteredEventHorizon);
                    } else {
                        // Should never happen, only a safety precaution :)
                        return Some(StoppingCriterion::OutOfBoundingBox(
                            self.state.spatial_position().to_cartesian(),
                        ));
                    }
                }
            };

        // Check if we crossed accretion disk
        if let Some(r) = black_hole.accretion_disk().check_intersection(
            self.state.spatial_position().to_cartesian(),
            state.spatial_position().to_cartesian(),
        ) {
            return Some(StoppingCriterion::CrossedAccretionDisk(r));
        };

        if state.r() > bounding_box_radius
            && state.spatial_position().dot(self.state.spatial_velocity()) > 0.
        {
            // We are very far from the black hole AND we are moving away from it
            // Then early stopping. We are going to infinity so use background color.
            return Some(StoppingCriterion::OutOfBoundingBox(
                state.spatial_position().to_cartesian(),
            ));
        }

        // We haven't converged yet. Keep the state to make a next step
        self.state = state;
        self.dλ = dλ;

        // Signal that we haven't converged by not giving any color
        None
    }

    pub fn get_color(
        &mut self,
        black_hole: BlackHole,
        bounding_box_radius: f64,
        skybox: Arc<Skybox>,
    ) -> Color {
        let mut accumulated_color = Color::new(0.0, 0.0, 0.0, 0.0);
        let mut transmittance = 1.0;

        for i in 0..crate::NUM_INTEGRATION_STEPS {
            if i > 0 && i % crate::NORMALIZATION_INTERVAL == 0 {
                self.state = self.state.renormalize(black_hole.radius());
            }

            if let Some(criterion) = self.step(black_hole, bounding_box_radius) {
                let hit_color = determine_color(&criterion, black_hole, &skybox);
                (accumulated_color, transmittance) =
                    blend(accumulated_color, hit_color, transmittance);
                if transmittance < 0.05 {
                    break;
                }
            }
        }

        let (final_color, _) = blend(accumulated_color, crate::BACKGROUND_COLOR, transmittance);

        gamma_correct(final_color)
    }
}
