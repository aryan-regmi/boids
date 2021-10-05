mod vectors;
pub use vectors::Vec2d;

mod boids;
pub use crate::boids::Boid;

mod world;
pub use world::{GlobalConstants, World};

mod collisions;
pub use collisions::{detect_collisions, raycast, Collision, Ray, Triangle};
