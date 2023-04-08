mod map;
mod mapgen;
mod modelgen;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) 
        .add_startup_system(setup)
        .add_system(player::player_move)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 0.5;

    let texture_handle = asset_server.load("C:/Users/wokste/Desktop/labyrinth_textures.png");

    let map = mapgen::make_map();

    // The actual map
    commands.spawn(PbrBundle {
        mesh: meshes.add( modelgen::map_to_mesh(&map)),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Opaque,
            unlit: true,
            ..default()
            //Color::WHITE.into()
        }),
        ..default()
    });

    // Player
    let player_pos = map.random_square();
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(player_pos.x as f32 + 0.5, 0.7, player_pos.z as f32 + 0.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(player::PlayerBundle::default());

    /*for i in 1 .. 20 {
        let monster_pos = map.random_square();
        commands.spawn(PbrBundle {
            mesh: meshes.add( Mesh),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                alpha_mode: AlphaMode::Opaque,
                unlit: true,
                ..default()
                //Color::WHITE.into()
            }),
            transform: Transform::from_xyz(monster_pos.x as f32 + 0.5, 0.5, monster_pos.z as f32 + 0.5).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }).insert(player::MonsterBundle {
            ..default()
        });
    }*/
}