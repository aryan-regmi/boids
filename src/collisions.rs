use crate::{Boid, GlobalConstants, Vec2d, World};

impl World {
    /// Implements warping around the borders of the world.
    pub fn warp_world_borders(&mut self) {
        for boid in self.boids.iter_mut() {
            if boid.position.x < -self.width {
                // If the boid hits the left wall, set position to the right wall
                boid.position.x = self.width;
            } else if boid.position.x > self.width {
                // If the boid hits the right wall, set position to the left wall
                boid.position.x = -self.width;
            } else if boid.position.y > self.height {
                // If the boid hits the top wall, set position to the bottom wall
                boid.position.y = -self.height;
            } else if boid.position.y < -self.height {
                // If the boid hits the bottom wall, set positon to the top
                boid.position.y = self.height;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Triangle {
    vertices: Vec<Vec2d>,
    edges: Vec<Ray>,
}

impl Triangle {
    pub fn new(boid_position: &Vec2d, boid_sprite_size: (f32, f32)) -> Self {
        let (width, height) = boid_sprite_size;
        let center = boid_position;

        // Calculate edge lengths from the sprite size
        let base = width;
        let sides = ((width / 2.).powi(2) + height.powi(2)).sqrt(); // Pythagorian rule to solve for hypotnuse length
        let edge_lengths = vec![sides, base, sides]; // Edges 1 and 3 represent the two sides while edge 2 is the base of the triangle.

        // Create (Isosocles) Triangle around the center, with the sprite dimensions
        // The vertices correspond with the order of the edges; vertex 1 forms the two sides, while
        // vertices 2 and 3 form the base
        // TODO: May have to fix the orientation of the hitbox
        let vertex1 = Vec2d::new(center.x, center.y + height / 2.);
        let vertex2 = Vec2d::new(center.x + width / 2., center.y - height / 2.);
        let vertex3 = Vec2d::new(center.x - width / 2., center.y - height / 2.);

        let vertices = vec![vertex1, vertex2, vertex3];
        let edges = Ray::from_vertices(&vertices, edge_lengths);

        Self { vertices, edges }
    }
}

/// Holds coefficients for the equation defining the ray.
/// (y = mx + b, from the start of the ray to the length)
/// slope is m ==> tan(theta) = tan(y/x)
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Ray {
    pub slope: Option<f32>,
    pub y_intercept: Option<f32>,
    pub start_position: Vec2d,
    pub length: f32,
}

impl Ray {
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

    fn vertical_line(start_position: Vec2d, ray_length: f32) -> Self {
        Self::new(None, None, start_position, ray_length)
    }

    fn from_vertices(vertices: &[Vec2d], edge_lengths: Vec<f32>) -> Vec<Self> {
        // Calculate and store all the edges
        let mut edges = vec![];
        for i in 0..vertices.len() {
            if i == vertices.len() - 1 {
                let slope = (vertices[0].y - vertices[i].y) / (vertices[0].x - vertices[i].x);

                // Make sure edge is not a vertical line
                if slope.is_finite() {
                    let y_intercept = vertices[i].y - slope * vertices[i].x;
                    let start_position = Vec2d::new(vertices[i].x, vertices[i].y);
                    edges.push(Ray::new(
                        Some(slope),
                        Some(y_intercept),
                        start_position,
                        edge_lengths[i],
                    ));
                } else {
                    let start_position = Vec2d::new(vertices[i].x, vertices[i].y);
                    edges.push(Ray::vertical_line(start_position, edge_lengths[i]));
                }
            } else {
                let slope =
                    (vertices[i + 1].y - vertices[i].y) / (vertices[i + 1].x - vertices[i].x);

                // Make sure edge is not a vertical line
                if slope.is_finite() {
                    let y_intercept = vertices[i].y - slope * vertices[i].x;
                    let start_position = Vec2d::new(vertices[i].x, vertices[i].y);
                    edges.push(Ray::new(
                        Some(slope),
                        Some(y_intercept),
                        start_position,
                        edge_lengths[i],
                    ));
                } else {
                    let start_position = Vec2d::new(vertices[i].x, vertices[i].y);
                    edges.push(Ray::vertical_line(start_position, edge_lengths[i]));
                }
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
                    let final_pos1 = &Vec2d::from_polar(self.length, slope.atan().to_degrees())
                        + &self.start_position;
                    let final_pos2 =
                        &Vec2d::from_polar(other.length, other_slope.atan().to_degrees())
                            + &other.start_position;

                    // If slopes are the same, the two rays are parallel and/or overlapping, not
                    // intersect perpendicularly
                    let diff = slope - other_slope;
                    if diff <= f32::EPSILON {
                        // Check for overlap
                        if Ray::overlap(
                            &self.start_position,
                            &other.start_position,
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
                        let y_intersect = -y_intersect;

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
                    let final_pos1 = &Vec2d::from_polar(self.length, slope.atan().to_degrees())
                        + &self.start_position;

                    // Safe to unwrap y-intercept, since if there is a slope then y-intercepts are
                    // guaranteed.
                    let b1 = self.y_intercept.unwrap();

                    // Plug in the x value of the other ray as a line to solve for intersect
                    let x_intersect = other.start_position.x;

                    // Solve for y_intersect by plugging in the x value of the other ray
                    let y_intersect = slope * x_intersect + b1;
                    let y_intersect = -y_intersect;

                    // Check that (x_intersect, y_intersect) lies along the ray and return it
                    // if does
                    if (x_intersect < final_pos1.x && x_intersect > self.start_position.x)
                        && (y_intersect > final_pos1.y && y_intersect < self.start_position.y)
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
                        let final_pos2 =
                            &Vec2d::from_polar(other.length, other_slope.atan().to_degrees())
                                + &other.start_position;

                        // Safe to unwrap y-intercept, since if there is a slope then y-intercepts are
                        // guaranteed.
                        let b2 = other.y_intercept.unwrap();

                        // Plug in the x value of the current ray as a line to solve for intersect
                        let x_intersect = self.start_position.x;

                        // Solve for y_intersect by plugging in the x value of the other ray
                        let y_intersect = other_slope * x_intersect + b2;
                        let y_intersect = -y_intersect;

                        // Check that (x_intersect, y_intersect) lies along the ray and return it
                        // if does
                        if (x_intersect < final_pos2.x && x_intersect > other.start_position.x)
                            && (y_intersect > final_pos2.y && y_intersect < other.start_position.y)
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

    pub fn contains(&self, point: Vec2d) -> bool {
        match self.slope {
            Some(m) => {
                let expected_y = -(m * point.x + self.y_intercept.unwrap());
                if point.y == expected_y {
                    return true;
                }
                false
            }
            // If current ray is a vertical line
            None => {
                let ray_endpoint = -(self.start_position.y + self.length);
                // If the point lines along the vertical ray
                if point.x == self.start_position.x
                    && point.y > ray_endpoint
                    && point.y < self.start_position.y
                {
                    return true;
                }
                false
            }
        }
    }
}

// TODO: Add FOV by limiting range of angles
/// Casts rays of speficifed length from the center of the given boid.
pub fn raycast(boid: &Boid, ray_length: f32, num_rays: u32) -> Vec<Ray> {
    // Cast rays from the center of the boid
    let center = &boid.position;

    // Separate rays in a 360 degree circle (tangent of this angle is the slope of the ray)
    let angle = 360. / num_rays as f32;

    // Create rays
    let mut rays = Vec::new(); // Stores the coefficients for the equation of the line/ray
    for i in 0..num_rays {
        // If it is a vertical line, set slope as undefined (tan is undef for 90 and 270 degrees)
        if angle == 90. || angle == 270. {
            rays.push(Ray {
                slope: None,
                y_intercept: None,
                start_position: center.clone(),
                length: ray_length,
            })
        } else {
            let ray_slope = (i as f32 * angle).to_radians().tan();

            // Calculate y-intercept of the ray
            let y_inter = center.y - (center.x * ray_slope);

            rays.push(Ray {
                slope: Some(ray_slope),
                y_intercept: Some(y_inter),
                start_position: center.clone(),
                length: ray_length,
            });
        }
    }
    rays
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

impl Circle {
    // Checks if current circle collides with other circle
    fn collides(&self, other: &Circle) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < self.radius + other.radius {
            return true;
        }

        false
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Collision {
    id: usize,
    location: Vec2d,
    velocity: Vec2d,
}

pub struct TriangleHitbox {
    triangle: Triangle,
    velocity: Vec2d,
}

pub fn detect_collisions(world: &mut World, globals: GlobalConstants) {
    // Grab values from globals
    let search_radius = globals.broad_radius;
    let ray_length = globals.boid_sight_range;
    let num_rays = globals.boid_sight_precision;

    // Grab boids to collide and their hitboxes
    let boids = world.boids.clone();

    for (boid_counter, boid) in boids.iter().enumerate() {
        // Broad search using circles
        let neighbours = broad_search(boid, world, search_radius);

        // Only do narrow search if there are neighbours/possible collisions
        if !neighbours.is_empty() {
            // Cast rays from boid
            let rays = raycast(boid, ray_length, num_rays);

            // Narrow search using raycast
            // -> Find point of intersection
            // -> Subtract distance from center to find actual collision

            // Loop through all possible collisions
            for (collision_id, neigh) in neighbours.iter().enumerate() {
                // Check if each edge of the neighbour's hitbox intersects any of our cast rays
                for neighbor_edge in &neigh.triangle.edges {
                    for ray in &rays {
                        // Check to see if the boid's rays intersect the neighbour
                        let intersect = ray.intersects(neighbor_edge);

                        // Check to see if the two boids intersect
                        if let Some(pos) = intersect {
                            // Check to see if intersect point exists on any of the current boid's edges
                            for edge in &world.hitboxes[boid_counter].edges {
                                if edge.contains(pos.clone()) {
                                    let collision = Collision {
                                        id: collision_id,
                                        location: pos.clone(),
                                        velocity: neigh.velocity.clone(),
                                    };
                                    world.boids[boid_counter].collisions.push(collision);

                                    dbg!("Collided");

                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn broad_search(boid: &Boid, world: &World, search_radius: f32) -> Vec<TriangleHitbox> {
    // Initalize necessary variables
    let mut possible_collisions: Vec<TriangleHitbox> = Vec::new();
    let other_boids = &world.boids;

    let center = &boid.position;
    let current_circle = Circle {
        x: center.x,
        y: center.y,
        radius: search_radius,
    };
    // Make circle and check if it overlaps any other boids
    for (neighbour_counter, other) in other_boids.iter().enumerate() {
        // Don't collide with itself
        if other != boid {
            let other_center = &other.position;
            let other_circle = Circle {
                x: other_center.x,
                y: other_center.y,
                radius: search_radius,
            };

            // If search radii collide, then there is a possible collision
            if current_circle.collides(&other_circle) {
                let hitbox = TriangleHitbox {
                    triangle: world.hitboxes[neighbour_counter].clone(),
                    velocity: other.velocity.clone(),
                };
                possible_collisions.push(hitbox);
            }
        }
    }

    possible_collisions
}

#[cfg(test)]
mod collision_tests {
    use super::*;

    #[test]
    fn it_can_create_new_triangle() {
        let p0 = Vec2d::new(0., 0.);
        let v0 = Vec2d::new(1., 1.);
        let boid = Boid::new(1., p0.clone(), v0);
        let tri = Triangle::new(&boid.position, (50., 50.));

        let vertices = tri.vertices;
        let edges = tri.edges;

        // Calculate edge lengths from the sprite size
        let base = 50.;
        let sides = ((50. / 2_f32).powi(2) + 50_f32.powi(2)).sqrt(); // Pythagorian rule to solve for hypotnuse length
        let edge_lengths = vec![sides, base, sides]; // Edges 1 and 3 represent the two sides while edge 2 is the base of the triangle.

        let vertex1 = Vec2d::new(p0.x, p0.y + 25.);
        let vertex2 = Vec2d::new(p0.x + 25., p0.y - 25.);
        let vertex3 = Vec2d::new(p0.x - 25., p0.y - 25.);

        let expected_edges = Ray::from_vertices(
            &[vertex1.clone(), vertex2.clone(), vertex3.clone()],
            edge_lengths,
        );

        assert_eq!(vertex1, vertices[0]);
        assert_eq!(vertex2, vertices[1]);
        assert_eq!(vertex3, vertices[2]);

        assert_eq!(edges, expected_edges);
    }

    #[test]
    fn it_can_check_ray_overlap() {
        let start_pos1 = Vec2d::new(0., 0.);
        let final_pos1 = Vec2d::new(0., 5.);
        let start_pos2 = Vec2d::new(0., 3.);
        let final_pos2 = Vec2d::new(0., 9.);
        assert!(Ray::overlap(
            &start_pos1,
            &start_pos2,
            &final_pos1,
            &final_pos2
        ));
    }

    #[test]
    fn it_can_check_ray_intersection() {
        // y = 3x
        let line1 = Ray::new(Some(3.), Some(0.), Vec2d::new(0., 0.), 10.);

        // y = -5x + 10
        let line2 = Ray::new(Some(5.), Some(10.), Vec2d::new(1., 1.), 10.);

        // x = 3
        let line3 = Ray::vertical_line(Vec2d::new(3., 0.), 100.);

        // x = 5
        let line4 = Ray::vertical_line(Vec2d::new(5., 0.), 10.);

        assert!(line1.intersects(&line2).is_some());
        assert!(line1.intersects(&line3).is_some());
        assert!(line3.intersects(&line4).is_none());
    }

    #[test]
    fn it_can_check_if_ray_contains_point() {
        let ray1 = Ray::vertical_line(Vec2d::new(3., 0.), 10.);
        let ray2 = Ray::new(Some(3.), Some(0.), Vec2d::new(0., 0.), 10.);
        let point = Vec2d::new(3., 9.);

        assert!(ray1.contains(point.clone()));
        assert!(ray2.contains(point));
    }
}
