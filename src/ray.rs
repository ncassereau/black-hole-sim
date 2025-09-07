use macroquad::prelude::*;
use std::collections::VecDeque;

use crate::{BlackHole, Draw, Scene};
use crate::{CartesianCoords4D, CartesianState4D, SphericalState4D};

const MEMORY_LENGTH: usize = 256;
const MEMORY_INTERVAL: usize = 1;
const INVERSE_MEM_LENGTH: f32 = 1. / MEMORY_LENGTH as f32;

pub struct Ray {
    state: SphericalState4D,
    memory: Option<VecDeque<CartesianCoords4D>>,
    memory_counter: usize,
}

impl Ray {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        dx: f64,
        dy: f64,
        dz: f64,
        reference: &CartesianCoords4D,
    ) -> Self {
        let (t_ref, x_ref, y_ref, z_ref) = reference.unpack();
        let state =
            CartesianState4D::cartesian(t_ref, x - x_ref, y - y_ref, z - z_ref, 0., dx, dy, dz);

        Self {
            state: state.to_spherical(),
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

    pub fn step(&mut self, black_hole: &BlackHole, dλ: f64) {
        if self.state.r() <= black_hole.radius() {
            return;
        }

        self.state = crate::geodesic::solve_geodesic(self.state, black_hole.radius(), dλ);

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
