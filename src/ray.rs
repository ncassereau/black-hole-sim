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

    pub fn step(&mut self, black_hole: BlackHole) -> Option<Color> {
        let radius = black_hole.radius() * 1.05;
        if self.state.r() <= radius {
            return Some(black_hole.color());
        }

        match crate::geodesic::solve_geodesic_rkf45(self.state, black_hole.radius(), self.dλ) {
            Ok((state, dλ)) => {
                self.state = state.renormalize(black_hole.radius());
                self.dλ = dλ;
            }
            Err(e) => {
                match e {
                    crate::geodesic::IntegrationError::MaxRetriesReached => println!("MAX RETRIES {:?}", self.state),
                    crate::geodesic::IntegrationError::MinStepReached => println!("MIN STEP"),
                };
                if self.state.r() < 1.5 * black_hole.radius() {
                    return Some(black_hole.color());
                } else {
                    return Some(crate::BACKGROUND_COLOR);
                }
            }
        };

        if self.state.r() <= radius || self.state.r() < 0. {
            return Some(black_hole.color());
        }
        let _ = self.push_to_memory(self.state.position().to_cartesian());
        None
    }

    pub fn get_color(&mut self, black_hole: BlackHole) -> Color {
        for _ in 0..crate::NUM_INTEGRATION_STEPS {
            if let Some(color) = self.step(black_hole) {
                return color;
            }
        }
        crate::BACKGROUND_COLOR
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
