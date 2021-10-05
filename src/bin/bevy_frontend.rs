use bevy::prelude::*;
use boids::{GlobalConstants, Vec2d, World};

struct GameState {
    world: World,
}

struct BoidEntity {}

struct Materials {
    boid_material: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    window: Res<Windows>,
) {
    // Load in Boid sprite
    let boid_sprite = asset_server.load("boid2.png");

    // Spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Add boids sprite to resources
    commands.insert_resource(Materials {
        boid_material: materials.add(boid_sprite.into()),
    });

    // Insert global state
    let globals = GlobalConstants {
        num_boids: 100,
        step_size: 0.01,
        max_speed: Vec2d::new(50., 50.),
        broad_radius: 20.,
        boid_sprite_size: (50., 50.),
        boid_sight_range: 10.,
        boid_sight_precision: 12,
    };
    commands.insert_resource(globals.clone());

    // Insert World
    let win = window.get_primary().unwrap();
    let (width, height) = (win.width(), win.height());
    commands.insert_resource(GameState {
        world: World::new(width, height, globals),
    });
}

fn spawn_boids(
    mut commands: Commands,
    game_state: ResMut<GameState>,
    globals: Res<GlobalConstants>,
    materials: Res<Materials>,
) {
    let boids = &game_state.world.boids;

    let (sprite_width, sprite_height) = globals.boid_sprite_size;

    for boid in boids {
        // Create transfrom to link internal boid and BoidEntity
        let mut transform =
            Transform::from_translation(Vec3::new(boid.position.x, boid.position.y, 0.));

        // Rotate transform (sprite) to face direction of internal boid
        transform.rotate(Quat::from_rotation_z(boid.position.direction()));

        // Spawn BoidEntity to mirror internal boid representation
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.boid_material.clone(),
                sprite: Sprite::new(Vec2::new(sprite_width, sprite_height)),
                transform,
                ..Default::default()
            })
            .insert(BoidEntity {});
    }
}

fn step_world(
    mut query: Query<&mut Transform, With<BoidEntity>>,
    mut game_state: ResMut<GameState>,
    globals: Res<GlobalConstants>,
) {
    // Step the internal world by specified step size
    game_state.world.warp_world_borders(); // Implements warping borders
    game_state.world.step(globals.step_size);

    // Update BoidEntity with boid info
    let boids = &game_state.world.boids;
    for (boid_id, mut transfrom) in query.iter_mut().enumerate() {
        let boid = &boids[boid_id];
        transfrom.translation.x = boid.position.x;
        transfrom.translation.y = boid.position.y;
        // There is a slight offset in the rotation of the sprite
        transfrom.rotation =
            Quat::from_rotation_z(boid.velocity.direction().to_radians() - 90_f32.to_radians());
    }
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Boids".to_string(),
            // TODO: Implement resizing window/world
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::ALICE_BLUE))
        .add_startup_system(setup.system())
        .add_startup_stage("Boid Setup", SystemStage::single(spawn_boids.system()))
        .add_system(step_world.system())
        .run()
}
