#![allow(dead_code)]

use crate::Vec2d;

/// Defines a bird-like object. This is the basic "entity" of this program.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Boid {
    pub position: Vec2d,
    pub velocity: Vec2d,
    pub mass: f32,
}

impl Boid {
    /// Creates a new boid from the specified mass, position, and velocity.
    pub fn new(mass: f32, inital_position: Vec2d, inital_velocity: Vec2d) -> Self {
        Self {
            position: inital_position,
            velocity: inital_velocity,
            mass,
        }
    }

    /// Moves the boid by a constat velocity every step size specified.
    pub fn constant_velocity_movement(&mut self, step_size: f32) {
        self.position += &self.velocity * step_size;
    }

    // Calculates side-slip angle (angle between body and its velocity vector).
    pub fn side_slip(&self) -> f32 {
        let vel_x = self.velocity.x;
        let vel_y = self.velocity.y;
        // Calculate angle the velocity vector points in
        let vel_angle = vel_y.atan2(vel_x).to_degrees();

        // Difference of velocity angle and heading is the side-slip angle
        let heading = self.position.direction();
        vel_angle - heading
    }
}

#[cfg(test)]
mod boid_tests {
    use super::*;

    #[test]
    fn it_can_make_boid() {
        let pos = Vec2d::new(0., 0.);
        let vel = Vec2d::new(3., 3.);
        let pos2 = Vec2d::new(0., 0.);
        let vel2 = Vec2d::new(3., 3.);
        let boid = Boid::new(1., pos, vel);
        let boid2 = Boid {
            mass: 1.,
            position: pos2,
            velocity: vel2,
        };

        assert!((boid.mass - boid2.mass).abs() < f32::EPSILON);
        assert_eq!(boid.position, boid2.position);
        assert_eq!(boid.velocity, boid2.velocity);
    }

    #[test]
    fn it_can_move_boid_constant_velocity() {
        let pos = Vec2d::new(0.0, 0.0);
        let vel = Vec2d::new(3.0, 3.0);
        let mut boid = Boid::new(1.0, pos, vel);
        boid.constant_velocity_movement(1.0);
        assert_eq!(boid.position, Vec2d::new(boid.position.x, -boid.position.y));
    }

    #[test]
    fn it_can_calculate_sideslip() {
        let pos = Vec2d::new(0.0, 0.0);
        let vel = Vec2d::new(3.0, 3.0);
        let boid = Boid::new(1.0, pos, vel);
        assert!(boid.side_slip() < f32::EPSILON);
    }
}
