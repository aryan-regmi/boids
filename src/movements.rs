use crate::Boid;
use bevy::prelude::{Mut, Quat, Transform};

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x_velocity: f32, y_velocity: f32) -> Self {
        Self {
            x: x_velocity,
            y: y_velocity,
        }
    }

    // heading is in degrees
    pub fn from_polar(magnitude: f32, heading: f32) -> Self {
        Self {
            x: magnitude * heading.to_radians().cos(),
            y: magnitude * heading.to_radians().sin(),
        }
    }

    // Returns heading of the velocity
    pub fn heading(&self) -> f32 {
        self.y.atan2(self.x)
    }
}

// Move with constant velocity
pub fn constant_velocity(transform: &mut Mut<Transform>, boid: &mut Boid, dt: f32) {
    // Add constant velocity to current positions
    transform.translation.x += dt * boid.velocity.x;
    transform.translation.y += dt * boid.velocity.y;

    // Rotate sprite in direction of motion (heading + 270 deg gives us correct sprite direction)
    transform.rotation =
        Quat::from_rotation_z(boid.velocity.heading() + (3. * core::f32::consts::PI / 2.));
}
