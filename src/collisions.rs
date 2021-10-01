use crate::World;

impl World {
    /// Implements warping around the borders of the world.
    pub fn warp_world_borders(&mut self) {
        for boid in self.boids.iter_mut() {
            if boid.position.x < 0. {
                // If the boid hits the left wall, set position to the right wall
                boid.position.x = self.width;
            } else if boid.position.x > self.width {
                // If the boid hits the right wall, set position to the left wall
                boid.position.x = 0.;
            } else if boid.position.y > 0. {
                // If the boid hits the top wall, set position to the bottom wall
                boid.position.y = -self.height;
            } else if boid.position.y < -self.height {
                // If the boid hits the bottom wall, set positon to the top
                boid.position.y = 0.
            }
        }
    }
}
