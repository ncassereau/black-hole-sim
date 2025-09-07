use macroquad::prelude::*;
use std::collections::VecDeque;

use crate::{BlackHole, Draw, Scene};
use crate::{CartesianCoords4D, CartesianState3D, SphericalState4D};

const MEMORY_LENGTH: usize = 256;
const MEMORY_INTERVAL: usize = 1;
const INVERSE_MEM_LENGTH: f32 = 1. / MEMORY_LENGTH as f32;

pub struct Ray {
    state: SphericalState4D,
    dλ: f64,
    memory: Option<VecDeque<CartesianCoords4D>>,
    memory_counter: usize,
}

impl Ray {
    pub fn new(
        spatial_state: CartesianState3D,
        reference: &CartesianCoords4D,
        rs: f64,
        dλ0: f64,
    ) -> Self {
        let (_, x_ref, y_ref, z_ref) = reference.unpack();
        let centered_state = CartesianState3D::cartesian(
            spatial_state.x() - x_ref,
            spatial_state.y() - y_ref,
            spatial_state.z() - z_ref,
            spatial_state.dx(),
            spatial_state.dy(),
            spatial_state.dz(),
        );

        Self {
            state: centered_state.to_spherical().to_4d(rs),
            dλ: dλ0,
            memory: Some(VecDeque::new()),
            memory_counter: 0,
        }
    }

    fn push_to_memory(&mut self, new_position: CartesianCoords4D) -> Result<(), ()> {
        if let Some(memory) = self.memory.as_mut() {
            self.memory_counter += 1;
            if self.memory_counter % MEMORY_INTERVAL == 0 {
                memory.push_back(new_position);
                if memory.len() > MEMORY_LENGTH {
                    memory.pop_front();
                }
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn step(&mut self, black_hole: &BlackHole) {
        if self.state.r() <= black_hole.radius() {
            return;
        }

        let (state, dλ, success) =
            crate::geodesic::solve_geodesic_rkf45(self.state, black_hole.radius(), self.dλ);

        if !success {
            return;
        }

        self.state = state;
        self.dλ = dλ;

        if state.r() <= black_hole.radius() || state.r() < 0. {
            return;
        }
        let _ = self.push_to_memory(self.state.position().to_cartesian());
    }
}

impl Draw for Ray {
    fn draw(&self, scene: &Scene) {
        if let Some(memory) = self.memory.as_ref()
            && memory.len() > 0
        {
            let mut current = scene.to_screen_coords(self.state.position().to_cartesian());

            for (index, next) in memory
                .iter()
                .rev()
                .map(|el| scene.to_screen_coords(*el))
                .enumerate()
            {
                let alpha = 1. - index as f32 * INVERSE_MEM_LENGTH;

                self.draw_line(current, next, 1., WHITE.with_alpha(alpha));

                current = next;
            }
        }
    }
}
