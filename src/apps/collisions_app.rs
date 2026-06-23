use bevy::prelude::*;
use rand::{rng, RngExt};

pub fn collision_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_system, ui_update, player_collisions));

    app
}

#[derive(Component)]
struct UI;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct PlayerStats {
    radius: f32,
    health: f32,
}

#[derive(Component)]
struct ObstacleStats {
    radius: f32,
    damage: f32,
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let obstacles_number: i16 = 50;

    commands.spawn(Camera2d);
    commands.spawn((
        Player,
        PlayerStats {
            radius: 30.0,
            health: 100.0,
        },
        Mesh2d(meshes.add(Circle::new(30.0))),
        // Color::srgb(1.0, 0.0, 0.0),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    let mut rng = rng();
    for _ in 0..obstacles_number {
        let radius = rng.random_range(2.0..40.0);
        let x = rng.random_range(-400.0..400.0);
        let y = rng.random_range(-300.0..300.0);
        commands.spawn((
            Obstacle,
            ObstacleStats {
                radius: radius,
                damage: 5.0,
            },
            Mesh2d(meshes.add(Circle::new(radius))),
            // Color::srgb(1.0, 0.0, 0.0),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    commands.spawn((UI, Text::new("".to_string())));
}

fn move_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if direction != Vec3::ZERO {
        let speed = 200.0;
        transform.translation += direction.normalize() * speed * time.delta_secs();
    }
}

fn ui_update(
    mut ui_query: Query<&mut Text, With<UI>>,
    player_query: Query<(&Transform, &PlayerStats), With<Player>>,
) {
    let Ok((transform, stats)) = player_query.single() else {
        return;
    };

    let Ok(mut text) = ui_query.single_mut() else {
        return;
    };

    *text = Text::new(format!(
        "position: {:.2}, {:.2}\nhealth: {:.2}",
        transform.translation.x, transform.translation.y, stats.health
    ));
}

fn player_collisions(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut PlayerStats), (With<Player>, Without<Obstacle>)>,
    obstacles_query: Query<(&Transform, &ObstacleStats), (With<Obstacle>, Without<Player>)>,
) {
    let Ok((transform, mut stats)) = player_query.single_mut() else {
        return;
    };

    for (obstacle_transform, obstacle_stats) in obstacles_query.iter() {
        let distance_sq = transform
            .translation
            .distance_squared(obstacle_transform.translation);
        let combined_radius = stats.radius + obstacle_stats.radius;
        if distance_sq < combined_radius * combined_radius {
            stats.health -= obstacle_stats.damage * time.delta_secs();
        }
    }
}
