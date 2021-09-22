mod boids;
pub use boids::{collision_system, move_boid, spawn_boids, Boid};

mod movements;
pub use movements::{constant_velocity, Velocity};

use bevy::prelude::*;
use rand::prelude::*;

pub struct Materials {
    pub boid_material: Handle<ColorMaterial>,
}

pub struct GlobalState {
    pub num_boids: usize,
    pub step_size: f32,
    pub max_velocity: Velocity,
    // Width and Height of Boid sprite
    pub boid_sprite_size: (f32, f32),
    // Detection radius of Boids
    pub detection_range: f32,
    // Determines elasticity of collisions
    pub boid_elasticity: f32,
}

fn randomize_velocity(rng: &mut rand::rngs::ThreadRng, max_velx: f32, max_vely: f32) -> (f32, f32) {
    // Randomize inital velocity between -max allowed and +max allowed velocity
    let mut vel_x: f32 = rng.gen();
    let mut vel_y: f32 = rng.gen();
    let x_sign: bool = rng.gen();
    let y_sign: bool = rng.gen();

    vel_x = match x_sign {
        true => vel_x * max_velx,
        false => -vel_x * max_velx,
    };

    vel_y = match y_sign {
        true => vel_y * max_vely,
        false => -vel_y * max_vely,
    };

    return (vel_x, vel_y);
}

// Implement warping
fn warping(windows: &Res<Windows>, transform: &mut Mut<Transform>) {
    // Get window borders
    let win = windows.get_primary().unwrap();
    let ceiling = win.height() / 2.;
    let ground = -win.height() / 2.;
    let left_wall = -win.width() / 2.;
    let right_wall = win.width() / 2.;

    // If boid hits right wall
    if transform.translation.x > right_wall {
        transform.translation.x = left_wall;
    }
    // If boid hits left wall
    else if transform.translation.x < left_wall {
        transform.translation.x = right_wall;
    }
    // If boid hits the the ceiling
    if transform.translation.y > ceiling {
        transform.translation.y = ground;
    }
    // If boid hits the ground
    else if transform.translation.y < ground {
        transform.translation.y = ceiling;
    }
}
