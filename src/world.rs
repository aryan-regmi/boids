#![allow(dead_code)]

use crate::{Boid, Triangle, Vec2d};
use rand::random;

/// Defines the whole world of the simulation.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct World {
    pub width: f32,
    pub height: f32,
    pub boids: Vec<Boid>,
    pub hitboxes: Vec<Triangle>,
}

/// Global state that holds all the constants for the `World`.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct GlobalConstants {
    // Number of boids in the world
    pub num_boids: usize,

    // Amount of time to step through each iteration/loop
    pub step_size: f32,

    // FIXME: Set speed to max in loop, if it exceeds the max
    // Maximum speeds of boids
    pub max_speed: Vec2d,

    // Sprite size of boids
    pub boid_sprite_size: (f32, f32),

    // Broad Search Radius
    pub broad_radius: f32,

    // How far the boids can see
    pub boid_sight_range: f32,

    // How many rays the boids cast
    pub boid_sight_precision: u32,
}

impl World {
    /// Creates new world object with specified size and boids.
    pub fn new(width: f32, height: f32, globals: GlobalConstants) -> Self {
        // Vector to store all boids
        let mut boids = Vec::new();

        // Vector to store the hitboxes for each boid
        let mut hitboxes = Vec::new();

        // Max speeds for boids
        let max_speed = &globals.max_speed;

        // Boid sprite size/hitbox size
        let boid_sprite_size = globals.boid_sprite_size;

        // Create new boids with randomized inital conditions
        for _i in 0..globals.num_boids {
            // Randomize inital conditions of boid
            let mass = random();
            let (inital_position, inital_velocity) =
                World::randomize_inital_conditions(width, height, max_speed);

            /* dbg!("Change This In Production!");
            let (inital_position, inital_velocity) = (Vec2d::new(100., 100.), Vec2d::new(0., 0.)); */

            // Add hitbox to world
            let hitbox = Triangle::new(&inital_position, boid_sprite_size);
            hitboxes.push(hitbox);

            // Add boid to world
            boids.push(Boid::new(mass, inital_position, inital_velocity));
        }

        Self {
            boids,
            hitboxes,
            width,
            height,
        }
    }

    fn randomize_inital_conditions(width: f32, height: f32, max_speed: &Vec2d) -> (Vec2d, Vec2d) {
        // Randomize signs of positions and velocities
        let mut signs = Vec::new();
        for i in 0..4 {
            let rand_sign = random::<bool>();
            if rand_sign {
                signs.push(1.);
            } else {
                signs.push(-1.)
            }
        }
        let x0 = signs[0] * width * random::<f32>();
        let y0 = signs[1] * height * random::<f32>();
        let vx0 = signs[2] * max_speed.x * random::<f32>();
        let vy0 = signs[3] * max_speed.y * random::<f32>();
        let inital_position = Vec2d::new(x0, y0);
        let inital_velocity = Vec2d::new(vx0, vy0);
        (inital_position, inital_velocity)
    }

    /// Steps the world through specified time step.
    pub fn step(&mut self, step_size: f32) {
        for boid in &mut self.boids {
            boid.constant_velocity_movement(step_size);
        }
    }
}

#[cfg(test)]
mod world_tests {
    use super::*;
    use crate::Vec2d;

    #[test]
    fn it_can_make_world() {
        let globals = GlobalConstants {
            num_boids: 1,
            step_size: 1.,
            max_speed: Vec2d::new(50., 50.),
            boid_sprite_size: (10., 10.),
            broad_radius: 20.,
            boid_sight_range: 10.,
            boid_sight_precision: 12,
        };
        let world = World::new(500., 500., globals);
        assert_eq!(world.width, 500.);
        assert_eq!(world.height, 500.);
        assert_eq!(world.boids.len(), 1);
        assert_eq!(world.hitboxes.len(), 1);
    }

    #[test]
    fn it_can_step_world() {
        let globals = GlobalConstants {
            num_boids: 1,
            step_size: 1.,
            max_speed: Vec2d::new(50., 50.),
            boid_sprite_size: (10., 10.),
            broad_radius: 20.,
            boid_sight_range: 10.,
            boid_sight_precision: 12,
        };
        let mut world = World::new(500., 500., globals);
        let init_pos = world.boids[0].position.clone();
        let init_vel = world.boids[0].velocity.clone();
        world.step(1.);
        assert_eq!(world.boids[0].position, init_pos + init_vel);
    }
}
