mod map;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) 
        .add_startup_system(setup_scene)
        .add_startup_system(setup_player)
        .add_system(player::player_move)
        .run();
}

/// set up a simple 3D scene
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.5;

    let texture_handle = asset_server.load("C:/Users/wokste/Desktop/labyrinth_textures.png");

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(32.0).into()),
        material: materials.add(Color::rgb(0., 0., 0.).into()),
        transform : Transform::from_xyz(16.0, 0.0, 16.0),
        ..default()
    });

    let map = map::make_map();

    // The actual map
    commands.spawn(PbrBundle {
        mesh: meshes.add( map::map_to_mesh(&map)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Opaque,
            unlit: true,
            ..default()
            //Color::WHITE.into()
        }),
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
        transform: Transform::from_xyz(7.0, 0.7, 22.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(player::PlayerBundle::default());
}