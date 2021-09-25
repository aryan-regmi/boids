#![allow(dead_code)]

use crate::{Position, Vec2d, Velocity};

/// Defines a bird-like object. This is the basic "entity" of this program.
#[derive(Debug)]
pub struct Boid {
    pub position: Position,
    pub velocity: Velocity,
    pub mass: f32,
}

impl Boid {
    /// Creates a new boid from the specified mass, position, and velocity.
    pub fn new(mass: f32, inital_position: Vec2d, inital_velocity: Vec2d) -> Self {
        Self {
            position: Position::new(inital_position.x, inital_position.y),
            velocity: Velocity::new(inital_velocity.x, inital_velocity.y),
            mass,
        }
    }

    /// Moves the boid by a constat velocity every step size specified.
    pub fn constant_velocity_movement(&mut self, step_size: f32) {
        self.position.vec += &self.velocity.vec * step_size;
    }
}

#[cfg(test)]
mod boid_tests {
    use super::*;

    #[test]
    fn it_can_make_boid() {
        let pos = Position::new(0., 0.);
        let vel = Velocity::new(3., 3.);
        let pos2 = Position::new(0., 0.);
        let vel2 = Velocity::new(3., -3.);
        let boid = Boid::new(1., pos.vec, vel.vec);
        let boid2 = Boid {
            mass: 1.,
            position: pos2,
            velocity: vel2,
        };

        assert_eq!(boid.mass, boid2.mass);
        assert_eq!(boid.position.vec, boid2.position.vec);
        assert_eq!(boid.velocity.vec, boid2.velocity.vec);
    }

    #[test]
    fn it_can_move_boid_constant_velocity() {
        let pos = Position::new(0.0, 0.0);
        let vel = Velocity::new(3.0, 3.0);
        let mut boid = Boid::new(1.0, pos.vec, vel.vec);
        boid.constant_velocity_movement(1.0);
        assert_eq!(
            boid.position.vec,
            Vec2d::new(boid.position.vec.x, boid.position.vec.y)
        );
    }
}
