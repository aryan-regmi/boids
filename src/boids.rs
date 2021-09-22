use crate::{constant_velocity, randomize_velocity, warping, GlobalState, Materials, Velocity};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

#[derive(Debug)]
pub struct Boid {
    pub velocity: Velocity,
}

// Spawns new boids
pub fn spawn_boids(mut commands: Commands, materials: Res<Materials>, globals: Res<GlobalState>) {
    // TODO: Randomize initial size too?
    // Seed for randomization
    let mut rng = rand::thread_rng();

    // Get Boid sprite size
    let (sprite_width, sprite_height) = globals.boid_sprite_size;

    // Spawn num_boids amount of Boids
    for _ in 0..globals.num_boids {
        // Randomize initial velocity
        let (vx0, vy0) =
            randomize_velocity(&mut rng, globals.max_velocity.x, globals.max_velocity.y);

        // Spawn boids
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.boid_material.clone(),
                sprite: Sprite::new(Vec2::new(sprite_width, sprite_height)),
                ..Default::default()
            })
            .insert(Boid {
                velocity: Velocity::new(vx0, vy0),
            });
    }
}

// Updates each boid's position
pub fn move_boid(
    windows: Res<Windows>,
    mut query: Query<(&mut Boid, &mut Transform)>,
    globals: Res<GlobalState>,
) {
    let dt = globals.step_size;
    for (mut boid, mut transform) in query.iter_mut() {
        // Implement warping
        warping(&windows, &mut transform);

        // Move w/ constant velocity
        constant_velocity(&mut transform, &mut boid, dt);
    }
}

#[derive(Debug)]
struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub fn collision_system(
    mut boid_query: Query<(&mut Boid, &Transform, &Sprite)>,
    collider_query: Query<(&Transform, &Sprite), With<Boid>>,
    globals: Res<GlobalState>,
) {
    // Loop through every other boid and check for collision
    for (mut boid, boid_transform, boid_sprite) in boid_query.iter_mut() {
        for (entity_transform, entity_sprite) in collider_query.iter() {
            // Check for collisions
            let collision = collide(
                boid_transform.translation,
                boid_sprite.size / 2.,
                entity_transform.translation,
                entity_sprite.size / 2.,
            );
            // MAKE IT SO FINAL VELOCITY IS AVG OF BOTH VELOCITIES

            // If there is collision, adjust velocity
            if let Some(collision) = collision {
                let elasticity = globals.boid_elasticity;
                let vel = &mut boid.velocity;
                // Reflect direction of velocity if collision pushes boid in opposite direction
                match collision {
                    Collision::Left => {
                        if vel.x > 0.0 {
                            vel.x = -elasticity * vel.x;
                        }
                    }

                    Collision::Right => {
                        if vel.x < 0.0 {
                            vel.x = -elasticity * vel.x;
                        }
                    }

                    Collision::Top => {
                        if vel.y < 0.0 {
                            vel.y = -elasticity * vel.y;
                        }
                    }

                    Collision::Bottom => {
                        if vel.y > 0.0 {
                            vel.y = -elasticity * vel.y;
                        }
                    }
                }
            }
        }
    }
}

// Checks to see if two rectangles are intersecting (colliding)
fn rect_intersect(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    let x1 = rect1.x;
    let x2 = rect2.x;
    let y1 = rect1.y;
    let y2 = rect2.y;
    let w1 = rect1.width;
    let w2 = rect2.width;
    let h1 = rect1.height;
    let h2 = rect2.height;

    if x1 + w1 < x2 || x2 + w2 < x1 || y1 + h1 < y2 || y2 + h2 < y1 {
        return false;
    } else {
        return true;
    }
}
