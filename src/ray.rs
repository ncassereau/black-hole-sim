use macroquad::prelude::*;
use ndarray::Array1;
use std::collections::VecDeque;

use crate::{BlackHole, Draw, Scene};

const MEMORY_LENGTH: usize = 256;
const MEMORY_INTERVAL: usize = 1;
const INVERSE_MEM_LENGTH: f32 = 1. / MEMORY_LENGTH as f32;

pub struct Ray {
    position: Array1<f64>,

    r: f64,
    phi: f64,
    dr: f64,
    dphi: f64,

    memory: Option<VecDeque<Array1<f64>>>,
    memory_counter: usize,
}

impl Ray {
    pub fn new(x: f64, y: f64, dx: f64, dy: f64, reference: &Array1<f64>) -> Self {
        let position = Array1::from_vec(vec![x, y]);
        let direction = Array1::from_vec(vec![dx, dy]);

        let centered = &position - reference;
        let r = f64::sqrt((&centered * &centered).sum());
        let phi = f64::atan2(centered[1], centered[0]);
        let dr = (&centered * &direction).sum() / r;
        let dphi = (centered[0] * direction[1] - centered[1] * direction[0]) / (r * r);

        Self {
            position,
            memory: Some(VecDeque::new()),
            memory_counter: 0,

            r,
            dr,
            phi,
            dphi,
        }
    }

    fn push_to_memory(&mut self, new_position: Array1<f64>) -> Result<(), &str> {
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
            Err("beurk")
        }
    }

    pub fn step(&mut self, black_hole: &BlackHole, dt: f64) {
        let old_position = self.position.clone();

        if self.r <= black_hole.radius() {
            return;
        } else {
            let (r, dr, phi, dphi) = crate::geodesic::solve_geodesic(
                self.r,
                self.dr,
                self.phi,
                self.dphi,
                black_hole.radius(),
                dt,
            );

            self.r = r;
            self.phi = phi;
            self.dr = dr;
            self.dphi = dphi;

            let x = self.r * f64::cos(self.phi);
            let y = self.r * f64::sin(self.phi);
            self.position = Array1::from_vec(vec![x, y]);
            let _ = self.push_to_memory(old_position);
        }
    }

    pub fn position(&self) -> &Array1<f64> {
        &self.position
    }
}

impl Draw for Ray {
    fn draw(&self, scene: &Scene) {
        if let Some(memory) = self.memory.as_ref()
            && memory.len() > 0
        {
            let mut current = scene.to_screen_coords(self.position());

            for (index, next) in memory
                .iter()
                .rev()
                .map(|el| scene.to_screen_coords(el))
                .enumerate()
            {
                let alpha = 1. - index as f32 * INVERSE_MEM_LENGTH;

                self.draw_line(&current, &next, 1., WHITE.with_alpha(alpha));

                current = next;
            }
        }
    }
}
