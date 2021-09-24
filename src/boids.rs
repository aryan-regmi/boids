#![allow(dead_code)]

use crate::{Pos, Vec2d, Vel};

struct Boid {
    position: Pos,
    velocity: Vel,
    mass: f32,
}

impl Boid {
    pub fn new(mass: f32, inital_position: Vec2d, inital_velocity: Vec2d) -> Self {
        Self {
            position: Pos::new(inital_position.x, inital_position.y),
            velocity: Vel::new(inital_velocity.x, inital_velocity.y),
            mass,
        }
    }
}
