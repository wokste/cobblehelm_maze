mod map;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .add_startup_system(setup_player)
        .add_system(player::player_move)
        .run();
}

/// set up a simple 3D scene
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let map = map::make_map();

    for y in 0 .. map.tiles.rows() {
        for x in 0 .. map.tiles.cols() {
            let tile = map.tiles[x][y];
            if tile.is_solid() && !tile.is_void() {
                // cube
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(tile.to_color().into()),
                    transform: Transform::from_xyz(x as f32, 0.5, y as f32),
                    ..default()
                });
            }
        }
    }
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}


/// set up a simple 3D scene
fn setup_player(
    mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 0.7, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(player::PlayerBundle::default());
}