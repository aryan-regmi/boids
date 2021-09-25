#![allow(dead_code)]

use crate::{Position, Vec2d, Velocity};

#[derive(Debug)]
pub struct Boid {
    pub position: Position,
    pub velocity: Velocity,
    pub mass: f32,
}

impl Boid {
    pub fn new(mass: f32, inital_position: Vec2d, inital_velocity: Vec2d) -> Self {
        Self {
            position: Position::new(inital_position.x, inital_position.y),
            velocity: Velocity::new(inital_velocity.x, inital_velocity.y),
            mass,
        }
    }

    pub fn constant_velocity_movement(&mut self, step_size: f32) {
        self.position.vec += &self.velocity.vec * step_size;
    }
}
