#![allow(dead_code)]
/// This module defines all the base vectors that our system is based on. All coordinates follow N-E-D (North-East-Down) (intertial) or H-R-D (Head-RightLimb-Down)
/// (body)
use std::ops::{Add, AddAssign, Mul, Sub};

// Generic 2d vector with x and y components.
#[derive(Debug)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

impl Vec2d {
    /// Creates new `Vec2d` with specified x and y components.
    fn new(x: f32, y: f32) -> Self {
        // y is negative because positive y-axix points down.
        Self { x, y: -y }
    }

    // Calculates dot product with another 2d vector.
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    // Calculates the cross product with another 2d vector.
    //
    // The vector points in the z-axis (into or out of the screen), but its magnitude (postive or
    // negative) tells us the rotation direction (counter-clockwise or clockwise respectively).
    pub fn cross(&self, other: &Self) -> f32 {
        self.x * -other.y - -self.y * other.x
    }

    // Calculates the vector normal to the current vector in-plane.
    //
    // Also known as the (left) surface normal of the vector; whereas the right-surface-normal
    // would switch the negative sign to the y value.
    pub fn surface_normal(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

// Enable using addition operator (addition of components)
impl Add for Vec2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Enable using subtraction operator (subtraction of components)
impl Sub for Vec2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Enable using multiplication operator (scalar multiplication)
impl Mul<f32> for &Vec2d {
    type Output = Vec2d;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Mul<&Vec2d> for f32 {
    type Output = Vec2d;

    fn mul(self, rhs: &Vec2d) -> Self::Output {
        Self::Output {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

// Enable using AddAssign (+=) operator
impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// Enable using equality (==) operator
impl PartialEq for Vec2d {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            return true;
        }
        false
    }
}

// Represents 2d position with x and y components.
#[derive(Debug)]
pub struct Position {
    pub vec: Vec2d,
}

impl Position {
    /// Creates new position vector.
    pub fn new(position_x: f32, position_y: f32) -> Self {
        Self {
            vec: Vec2d::new(position_x, position_y),
        }
    }

    // Calculates the angle (in degrees) between North and the postion vector.
    pub fn heading(&self) -> f32 {
        let x_pos = self.vec.x;
        let y_pos = self.vec.y;

        y_pos.atan2(x_pos).to_degrees()
    }
}

// Represents 2d velocity.
#[derive(Debug)]
pub struct Velocity {
    pub vec: Vec2d,
}

impl Velocity {
    // Creates new velocity vector from given x and y components.
    pub fn new(velocity_x: f32, velocity_y: f32) -> Self {
        Self {
            vec: Vec2d::new(velocity_x, velocity_y),
        }
    }

    // Creates new velocity vector from given angle and magnitude.
    //
    // Parameters:
    // ------------
    // magnitude - The magnitude of the velocity vector (speed).
    //
    // angle - The angle of the velocity vector. MUST BE IN DEGREES!
    pub fn from_polar(magnitude: f32, angle: f32) -> Self {
        Self {
            vec: Vec2d::new(
                magnitude * angle.to_radians().cos(),
                magnitude * angle.to_radians().sin(),
            ),
        }
    }

    // Calculates side-slip angle (angle between body and its velocity vector).
    //
    // Parameters:
    // ------------
    // heading - The angle of the body and North (angle of position vector), in [Degrees].
    pub fn side_slip(&self, heading: f32) -> f32 {
        let vel_x = self.vec.x;
        let vel_y = self.vec.y;
        // Calculate angle the velocity vector points in
        let vel_angle = vel_y.atan2(vel_x).to_degrees();

        // Difference of velocity angle and heading is the side-slip angle
        vel_angle - heading
    }
}

#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn it_can_make_vec2d() {
        let vec1 = Vec2d::new(0.1, 0.5);
        let vec2 = Vec2d { x: 0.1, y: -0.5 };
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn it_can_calculate_dot_product_of_vec2d() {
        let vec1 = Vec2d::new(0., 1.);
        let vec2 = Vec2d::new(1., 0.);
        let dot = vec1.dot(&vec2);

        assert_eq!(dot, 0.);
    }

    #[test]
    fn it_can_calculate_cross_product_of_vec2d() {
        let vec1 = Vec2d::new(1., 0.);
        let vec2 = Vec2d::new(0., 1.);
        let cross = vec1.cross(&vec2);
        assert_eq!(cross, 1.);
    }

    #[test]
    fn it_can_calculate_surface_normal_of_vec2d() {
        let vec = Vec2d::new(0., 1.);
        let normal = vec.surface_normal();
        assert_eq!(normal, Vec2d::new(1., 0.))
    }

    #[test]
    fn it_can_add_vec2() {
        let vec1 = Vec2d::new(1., 0.);
        let vec2 = Vec2d::new(0., 1.);
        let added = Vec2d::new(1., 1.);
        assert_eq!(added, vec1 + vec2);
    }

    #[test]
    fn it_can_subtract_vec2() {
        let vec1 = Vec2d::new(1., 0.);
        let vec2 = Vec2d::new(0., 1.);
        let sub = Vec2d::new(1., -1.);
        assert_eq!(sub, vec1 - vec2);
    }

    #[test]
    fn it_can_multiply_scalar_into_vec2d() {
        let vec1 = Vec2d::new(1.0, 1.0);
        let scaled = 10. * &vec1;
        assert_eq!(scaled, Vec2d::new(10., 10.))
    }

    #[test]
    fn test_equality_for_vec2d() {
        let vec1 = Vec2d::new(1.0, 3.5);
        let vec2 = Vec2d::new(1.0, 3.5);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn it_can_add_assign_vec2d() {
        let mut vec1 = Vec2d::new(1.0, 3.5);
        vec1 += Vec2d::new(2.5, 0.);
        assert_eq!(vec1, Vec2d::new(3.5, 3.5));
    }

    #[test]
    fn it_can_create_position_vector() {
        let pos = Position::new(3., 5.);
        assert_eq!(pos.vec, Vec2d { x: 3., y: -5. });
    }

    #[test]
    fn it_can_calculate_heading() {
        let pos = Position::new(1.0, 1.0);
        assert_eq!(pos.heading(), -45.0);
    }

    #[test]
    fn it_can_create_velocity_vector() {
        let v1 = Velocity::new(1., 1.);
        let v2 = Velocity::from_polar(2_f32.sqrt(), 45.0);
        let diff = v1.vec - v2.vec;
        assert!(diff.x < f32::EPSILON);
        assert!(diff.y < f32::EPSILON);
    }

    #[test]
    fn it_can_calculate_side_slip() {
        let v1 = Velocity::new(5.3, 5.3);
        let pos = Position::new(1.0, 1.0);
        assert_eq!(v1.side_slip(pos.heading()), 0.);
    }
}
