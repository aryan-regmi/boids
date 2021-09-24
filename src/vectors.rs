#![allow(dead_code)]
/// All coordinates follow N-E-D (North-East-Down) (intertial) or H-R-D (Head-RightLimb-Down)
/// (body)
use std::ops::{Add, Sub};

// Generic 2d vector
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

impl Vec2d {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
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
        self.x * other.y - self.y * other.x
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
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Enable using subtraction operator (subtraction of components)
impl Sub for Vec2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Represents 2d position.
pub struct Pos {
    pub position: Vec2d,
}

impl Pos {
    pub fn new(position_x: f32, position_y: f32) -> Self {
        Self {
            position: Vec2d::new(position_x, position_y),
        }
    }

    // Calculates the angle (in degrees) between North and the postion vector.
    pub fn heading(&self) -> f32 {
        let x_pos = self.position.x;
        let y_pos = self.position.y;

        y_pos.atan2(x_pos).to_degrees()
    }
}

// Represents 2d velocity.
pub struct Vel {
    pub velocity: Vec2d,
}

impl Vel {
    // Creates new velocity vector from given x and y components.
    pub fn new(velocity_x: f32, velocity_y: f32) -> Self {
        Self {
            velocity: Vec2d::new(velocity_x, velocity_y),
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
            velocity: Vec2d::new(
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
        let vel_x = self.velocity.x;
        let vel_y = self.velocity.y;
        // Calculate angle the velocity vector points in
        let vel_angle = vel_y.atan2(vel_x).to_degrees();

        // Difference of velocity angle and heading is the side-slip angle
        vel_angle - heading
    }
}
