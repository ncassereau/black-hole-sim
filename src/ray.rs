
use macroquad::prelude::*;
use ndarray::Array1;
use std::collections::VecDeque;

use crate::{Draw, Scene};

const SPEED_OF_LIGHT: f64 = 300_000_000.;
const MEMORY_LENGTH: usize = 256;
const MEMORY_INTERVAL: usize = 1;
const INVERSE_MEM_LENGTH: f32 = 1. / MEMORY_LENGTH as f32;

pub struct Ray {
    
    position: Array1<f64>,
    direction: Array1<f64>,
    memory: Option<VecDeque<Ray>>,
    memory_counter: usize,

}

impl Ray {

    pub fn new(x: f64, y: f64, dx: f64, dy: f64) -> Self {
        Self {
            position: Array1::from_vec(vec![x, y]),
            direction: Array1::from_vec(vec![dx, dy]),
            memory: Some(VecDeque::new()),
            memory_counter: 0,
        }
    }

    fn push_to_memory(&mut self, ray: Self) -> Result<(), &str> {
        if let Some(memory) = self.memory.as_mut() {
            self.memory_counter += 1;
            if self.memory_counter % MEMORY_INTERVAL == 0 {
                memory.push_back(ray);
                if memory.len() > MEMORY_LENGTH {
                    memory.pop_front();
                }
            }
            Ok(())
        } else {
            Err("beurk")
        }
    }

    pub fn step(&mut self) {
        let old_position = self.position.clone();
        self.position = &self.position + &self.direction * SPEED_OF_LIGHT;

        let new_ray = Self {
            position: old_position,
            direction: self.direction.clone(),
            memory: None,
            memory_counter: 0,
        };
        let _ = self.push_to_memory(new_ray);
    }

    pub fn position(&self) -> &Array1<f64> {
        &self.position
    }

}

impl Draw for Ray {

    fn draw(&self, scene: &Scene) {

        if let Some(memory) = self.memory.as_ref() && memory.len() > 0 {
            let mut mapped_mem = memory.iter().map(|el| scene.to_screen_coords(el.position()));

            // We go through the trail from the end to the beginning.
            // But it should evanescing from beginning to end.
            // Therefore alpha should from range from 0 to 1 as we get to head of trail.
            let memory_length = (memory.len() as i32 - 2) as f32;
            let alpha_factor = 1.0 - memory_length * INVERSE_MEM_LENGTH;

            if let Some(mut current) = mapped_mem.next() {
                for (index, next) in mapped_mem.enumerate() {
                    let alpha = alpha_factor - index as f32 * INVERSE_MEM_LENGTH;

                    self.draw_line(
                        &current,
                        &next,
                        1.,
                        WHITE.with_alpha(alpha),
                    );

                    current = next;
                }
            }
        }

    }

}
