
use macroquad::prelude::*;
use ndarray::Array1;
use std::collections::VecDeque;

use crate::{Draw, Scene};

const SPEED_OF_LIGHT: f64 = 300_000_000.;
const MEMORY_LENGTH: usize = 256;
const MEMORY_INTERVAL: usize = 1;

pub struct Ray {
    
    position: Array1<f64>,
    direction: Array1<f64>,
    memory: VecDeque<Array1<f64>>,
    memory_counter: usize,

}

impl Ray {

    pub fn new(x: f64, y: f64, dx: f64, dy: f64) -> Self {
        Self {
            position: Array1::from_vec(vec![x, y]),
            memory: VecDeque::new(),
            memory_counter: 0,
            direction: Array1::from_vec(vec![dx, dy]),
        }
    }

    pub fn step(&mut self) {
        self.position = &self.position + &self.direction * SPEED_OF_LIGHT;
        self.memory_counter += 1;
        if self.memory_counter % MEMORY_INTERVAL == 0 {
            self.memory.push_back(self.position.clone());
            if self.memory.len() > MEMORY_LENGTH {
                self.memory.pop_front();
            }
        }
    }

    pub fn position(&self) -> &Array1<f64> {
        &self.position
    }

}

impl Draw for Ray {

    fn draw(&self, scene: &Scene) {
        {
            let centered = scene.to_screen_coords(&self.position);
            self.draw_circle(centered, 5., WHITE);
        }

        if self.memory.len() == 0 {
            return ()
        }

        for index in 1..self.memory.len() {
            let index_from_start = self.memory.len() - 1 - index;
            let first_position = &self.memory[index_from_start];
            let second_position = &self.memory[index_from_start + 1];
            let centered_first = scene.to_screen_coords(first_position);
            let centered_second = scene.to_screen_coords(second_position);
            let alpha = 1. - (index as f32) / MEMORY_LENGTH as f32;
            self.draw_line(
                centered_first,
                centered_second,
                1.,
                WHITE.with_alpha(alpha),
            );
        }
    }

}
