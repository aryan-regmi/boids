#![allow(dead_code)]

use crate::Boid;

#[derive(Debug)]
pub struct World {
    width: f32,
    height: f32,
    boids: Vec<Boid>,
}

#[derive(Debug)]
struct GlobalState {
    // Number of boids
    num_boids: usize,

    // Amount of time to step through each iteration/loop
    step_size: f32,
}

impl World {
    pub fn new(width: f32, height: f32, boids: Vec<Boid>) -> Self {
        Self {
            width,
            height,
            boids,
        }
    }

    pub fn step(&mut self, step_size: f32) {
        for boid in &mut self.boids {
            boid.constant_velocity_movement(step_size);
        }
    }
}
