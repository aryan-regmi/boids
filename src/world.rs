use crate::Boid;

#[derive(Debug)]
pub struct World {
    width: f32,
    height: f32,
    boids: Vec<Boid>,
}

#[derive(Debug)]
struct GlobalState {
    // Number of boids
    num_boids: usize,

    // Amount of time to step through each iteration/loop
    step_size: f32,
}
