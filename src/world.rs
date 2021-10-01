#![allow(dead_code)]

use crate::Boid;

/// Defines the whole world of the simulation.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct World {
    pub width: f32,
    pub height: f32,
    pub boids: Vec<Boid>,
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

mod world_tests {
    use super::*;
    use crate::Vec2d;

    #[test]
    fn it_can_make_world() {
        let boid = Boid::new(1., Vec2d::new(0., 0.), Vec2d::new(0., 0.));
        let world = World::new(500., 500., vec![boid.clone()]);
        assert_eq!(
            world,
            World {
                width: 500.,
                height: 500.,
                boids: vec![boid]
            }
        )
    }

    #[test]
    fn it_can_step_world() {
        let boid = Boid::new(1., Vec2d::new(0., 0.), Vec2d::new(1., 0.));
        let mut world = World::new(500., 500., vec![boid]);
        world.step(1.);
        assert_eq!(world.boids[0].position, Vec2d::new(1., 0.));
    }
}
