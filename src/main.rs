use bevy::prelude::*;
use boids::{collision_system, move_boid, spawn_boids, GlobalState, Materials, Velocity};

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Load in Boid sprite
    let boids_sprite = asset_server.load("/home/aryan/Documents/Dev/rust/boids/assets/boid2.png");

    // Spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Add boids sprite to resources
    commands.insert_resource(Materials {
        boid_material: materials.add(boids_sprite.into()),
    });

    // Insert global state
    commands.insert_resource(GlobalState {
        num_boids: 5,
        step_size: 0.1,
        max_velocity: Velocity::new(25.0, 25.0),
        boid_sprite_size: (50.0, 50.0),
        detection_range: 0.001,
        boid_elasticity: 0.3,
    });
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Boids".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::ALICE_BLUE))
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_boids.system()))
        .add_system(collision_system.system())
        .add_system(move_boid.system())
        .run();
}
