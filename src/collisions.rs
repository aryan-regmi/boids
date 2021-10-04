use crate::{Boid, Vec2d, World};

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

pub struct Triangle {
    vertices: Vec<Vec2d>,
    edges: Vec<RayEqn>,
}

impl Triangle {
    pub fn new(vertex1: Vec2d, vertex2: Vec2d, vertex3: Vec2d, ray_length: f32) -> Self {
        let vertices = vec![vertex1, vertex2, vertex3];
        let edges = RayEqn::from_vertices(&vertices, ray_length);

        Self { vertices, edges }
    }
}

/// Holds coefficients for the equation defining the ray.
/// (y = mx + b, from the start of the ray to the length)
/// slope is m ==> tan(theta) = tan(y/x)
#[derive(Debug)]
pub struct RayEqn {
    pub slope: Option<f32>,
    pub y_intercept: Option<f32>,
    pub start_position: Vec2d,
    pub length: f32,
}

impl RayEqn {
    pub fn new(
        slope: Option<f32>,
        y_intercept: Option<f32>,
        start_position: Vec2d,
        length: f32,
    ) -> Self {
        Self {
            slope,
            y_intercept,
            start_position,
            length,
        }
    }

    pub fn from_vertices(vertices: &[Vec2d], ray_length: f32) -> Vec<Self> {
        // Calculate and store all the edges
        let mut edges = vec![];
        for i in 0..vertices.len() {
            if i == vertices.len() - 1 {
                let slope = (vertices[0].y - vertices[i].y) / (vertices[0].x - vertices[i].x);
                let y_intercept = vertices[i].y - slope * vertices[i].x;
                let start_position = Vec2d::new(vertices[i].x, vertices[i].y);
                edges.push(RayEqn::new(
                    Some(slope),
                    Some(y_intercept),
                    start_position,
                    ray_length,
                ));
                /* let edge = &vertices[0] - &vertices[i];
                edges.push(edge) */
            } else {
                let slope =
                    (vertices[i + 1].y - vertices[i].y) / (vertices[i + 1].x - vertices[i].x);
                let y_intercept = vertices[i].y - slope * vertices[i].x;
                let start_position = Vec2d::new(vertices[i].x, vertices[i].y);
                edges.push(RayEqn::new(
                    Some(slope),
                    Some(y_intercept),
                    start_position,
                    ray_length,
                ));
            }
        }

        edges
    }

    pub fn overlap(
        start_pos1: &Vec2d,
        start_pos2: &Vec2d,
        final_pos1: &Vec2d,
        final_pos2: &Vec2d,
    ) -> bool {
        // Initial x & y positions of first ray
        let (min_x1, min_y1) = (start_pos1.x, start_pos1.y);
        // Final x & y positions of first ray
        let (max_x1, max_y1) = (final_pos1.x, final_pos1.y);

        // Initial x & y positions of second ray
        let (min_x2, min_y2) = (start_pos2.x, start_pos2.y);
        // Final x & y positions of second ray
        let (max_x2, max_y2) = (final_pos2.x, final_pos2.y);

        // Check if both x and y values overlap
        !((max_x2 < min_x1 || max_x1 < min_x2) && (max_y2 < min_y1 || max_y1 < min_y2))
    }

    pub fn intersects(&self, other: &Self) -> Option<Vec2d> {
        match self.slope {
            Some(slope) => match other.slope {
                // If current ray has a defined slope
                Some(other_slope) => {
                    // Calculate final positions of each ray
                    let final_pos1 =
                        &Vec2d::from_polar(self.length, slope.atan()) + self.start_position;
                    let final_pos2 =
                        &Vec2d::from_polar(other.length, other_slope.atan()) + other.start_position;

                    // If slopes are the same, the two rays are parallel and/or overlapping, not
                    // intersect perpendicularly
                    let diff = slope - other_slope;
                    if diff <= f32::EPSILON {
                        // Check for overlap
                        if RayEqn::overlap(
                            self.start_position,
                            other.start_position,
                            &final_pos1,
                            &final_pos2,
                        ) {
                            // If it overlaps, the difference between the max values is the point
                            // of intersection
                            return Some(final_pos2 - final_pos1);
                        }
                        None
                    } else {
                        // Safe to unwrap y-intercept, since if there is a slope then y-intercepts are
                        // guaranteed.
                        let b1 = self.y_intercept.unwrap();
                        let b2 = other.y_intercept.unwrap();

                        // If slopes are different, simply solve for intersection of two lines
                        let x_intersect = (b2 - b1) / (slope - other_slope);
                        let y_intersect = slope * x_intersect + b1; // Plug in x_intersect to ray equation

                        // Check that (x_intersect, y_intersect) lies along the ray and return it
                        // if does
                        if (x_intersect < final_pos1.x && x_intersect > self.start_position.x)
                            && (y_intersect < final_pos1.y && y_intersect > self.start_position.y)
                        {
                            Some(Vec2d::new(x_intersect, y_intersect))
                        } else {
                            None
                        }
                    }
                }
                // If other slope is undefined, it is a vertical line
                None => {
                    // Endpoint of ray
                    let final_pos1 =
                        &Vec2d::from_polar(self.length, slope.atan()) + self.start_position;

                    // Safe to unwrap y-intercept, since if there is a slope then y-intercepts are
                    // guaranteed.
                    let b1 = self.y_intercept.unwrap();

                    // Plug in the x value of the other ray as a line to solve for intersect
                    let x_intersect = other.start_position.x;

                    // Solve for y_intersect by plugging in the x value of the other ray
                    let y_intersect = slope * x_intersect + b1;

                    // Check that (x_intersect, y_intersect) lies along the ray and return it
                    // if does
                    if (x_intersect < final_pos1.x && x_intersect > self.start_position.x)
                        && (y_intersect < final_pos1.y && y_intersect > self.start_position.y)
                    {
                        Some(Vec2d::new(x_intersect, y_intersect))
                    } else {
                        None
                    }
                }
            },
            // If current ray has undefined slope (vertical line)
            None => {
                // Algo is the same as the other `None` branch, except the two rays are switched
                match other.slope {
                    // Other ray has a defines slope
                    Some(other_slope) => {
                        // Endpoint of ray
                        let final_pos2 = &Vec2d::from_polar(other.length, other_slope.atan())
                            + other.start_position;

                        // Safe to unwrap y-intercept, since if there is a slope then y-intercepts are
                        // guaranteed.
                        let b2 = other.y_intercept.unwrap();

                        // Plug in the x value of the current ray as a line to solve for intersect
                        let x_intersect = self.start_position.x;

                        // Solve for y_intersect by plugging in the x value of the other ray
                        let y_intersect = other_slope * x_intersect + b2;

                        // Check that (x_intersect, y_intersect) lies along the ray and return it
                        // if does
                        if (x_intersect < final_pos2.x && x_intersect > other.start_position.x)
                            && (y_intersect < final_pos2.y && y_intersect > other.start_position.y)
                        {
                            Some(Vec2d::new(x_intersect, y_intersect))
                        } else {
                            None
                        }
                    }

                    // Both rays are vertical and will never intersect
                    None => None,
                }
            }
        }
    }
}

// TODO: Add FOV by limiting range of angles
/// Casts rays of speficifed length from the center of the given boid.
pub fn raycast(boid: &Boid, ray_length: f32, num_rays: u32) {
    // Cast rays from the center of the boid
    let center = &boid.position;

    // Separate rays in a 360 degree circle (tangent of this angle is the slope of the ray)
    let angle = 360. / num_rays as f32;

    // Create rays
    let mut rays = Vec::new(); // Stores the coefficients for the equation of the line/ray
    for i in 0..num_rays {
        // If it is a vertical line, set slope as undefined (tan is undef for 90 and 270 degrees)
        if angle == 90. || angle == 270. {
            rays.push(RayEqn {
                slope: None,
                y_intercept: None,
                start_position: center,
                length: ray_length,
            })
        } else {
            let ray_slope = (i as f32 * angle).to_radians().tan();

            // Calculate y-intercept of the ray
            let y_inter = center.y - (center.x * ray_slope);

            rays.push(RayEqn {
                slope: Some(ray_slope),
                y_intercept: Some(y_inter),
                start_position: center,
                length: ray_length,
            });
        }
    }
}
