use macroquad::prelude::*;

use crate::BlackHole;
use crate::{CartesianState3D, SphericalState4D};

pub enum StoppingCriterion {
    EnteredEventHorizon,
    OutOfBoundingBox,
    CrossedAccretionDisk(f64),
}

fn determine_color(stopping_criterion: StoppingCriterion, black_hole: BlackHole) -> Color {
    match stopping_criterion {
        StoppingCriterion::EnteredEventHorizon => black_hole.color(),
        StoppingCriterion::OutOfBoundingBox => crate::BACKGROUND_COLOR,
        StoppingCriterion::CrossedAccretionDisk(r) => {
            black_hole.accretion_disk().get_color(r).unwrap()
        } // We can unwrap safely because this Criterion is only trigged when we are within the accretion disk
    }
}

pub struct Ray {
    state: SphericalState4D,
    dλ: f64,
}

impl Ray {
    pub fn new(spatial_state: CartesianState3D, rs: f64, dλ0: f64) -> Self {
        let centered_state = CartesianState3D::cartesian(
            spatial_state.x(),
            spatial_state.y(),
            spatial_state.z(),
            spatial_state.dx(),
            spatial_state.dy(),
            spatial_state.dz(),
        );
        Self {
            state: centered_state.to_spherical().to_4d(rs),
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
                Ok((state, dλ)) => {
                    let state = state.renormalize(black_hole.radius());
                    (state, dλ)
                }
                Err(_) => {
                    if self.state.r() < black_hole.visual_radius() {
                        // RKF Step failed because we are very close to Black Hole. We therefore consider that we fell into it.
                        return Some(StoppingCriterion::EnteredEventHorizon);
                    } else {
                        // Should never happen, only a safety precaution :)
                        return Some(StoppingCriterion::OutOfBoundingBox);
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
            return Some(StoppingCriterion::OutOfBoundingBox);
        }

        // We haven't converged yet. Keep the state to make a next step
        self.state = state;
        self.dλ = dλ;

        // Signal that we haven't converged by not giving any color
        None
    }

    pub fn get_color(&mut self, black_hole: BlackHole, bounding_box_radius: f64) -> Color {
        let stopping_criterion = {
            let mut counter = 0;
            loop {
                if let Some(criterion) = self.step(black_hole, bounding_box_radius) {
                    break criterion;
                }
                counter += 1;
                if counter > crate::NUM_INTEGRATION_STEPS {
                    break StoppingCriterion::OutOfBoundingBox;
                }
            }
        };
        determine_color(stopping_criterion, black_hole)
    }
}
