#![allow(dead_code)]

use crate::Boid;

/// Defines the whole world of the simulation.
#[derive(Debug)]
pub struct World {
    width: f32,
    height: f32,
    boids: Vec<Boid>,
}

/// Global state that holds all the constants for the `World`.
#[derive(Debug)]
struct GlobalState {
    // Number of boids in the world
    num_boids: usize,

    // Amount of time to step through each iteration/loop
    step_size: f32,
}

impl World {
    /// Creates new world object with specified size and boids.
    pub fn new(width: f32, height: f32, boids: Vec<Boid>) -> Self {
        Self {
            width,
            height,
            boids,
        }
    }

    /// Steps the world through specified time step.
    pub fn step(&mut self, step_size: f32) {
        for boid in &mut self.boids {
            boid.constant_velocity_movement(step_size);
        }
    }
}
