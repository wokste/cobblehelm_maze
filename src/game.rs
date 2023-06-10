use bevy::prelude::*;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]

pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
    Paused,
//    VendingMachine,
}

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_startup_system(start_game.after(crate::app_setup))
            .insert_resource(crate::map::MapData::default())
            .insert_resource(crate::rendering::SpriteResource::default())
            .insert_resource(crate::GameInfo::default())
            .add_system(crate::player::player_input.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::player::update_map.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::ai::ai_los.in_set(OnUpdate(GameState::InGame)).after(crate::player::update_map))
            .add_system(crate::ai::ai_fire.in_set(OnUpdate(GameState::InGame)).after(crate::ai::ai_los))
            .add_system(crate::physics::do_physics.in_set(OnUpdate(GameState::InGame)).after(crate::player::player_input))
            .add_system(crate::weapon::check_projectile_creature_collisions.in_set(OnUpdate(GameState::InGame)))
            .add_system(crate::weapon::fire_weapons.in_set(OnUpdate(GameState::InGame)).after(crate::player::player_input).after(crate::ai::ai_fire))
    
            .add_system(crate::rendering::face_camera.in_set(OnUpdate(GameState::InGame)).after(crate::physics::do_physics))
            .add_system(crate::rendering::animate_sprites.in_set(OnUpdate(GameState::InGame)))
            ;
    }
}


/// set up the level
fn start_game(
    mut commands: Commands,
    mut map_data: ResMut<crate::map::MapData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res: ResMut<crate::rendering::SpriteResource>,
) {

    make_level(fastrand::u8(1..=5), &mut commands, &mut map_data, &mut meshes, &mut render_res);
}


/// set up the game
fn make_level(
    level : u8,
    commands: &mut Commands,
    map_data: &mut ResMut<crate::map::MapData>,
    meshes: &mut ResMut<Assets<Mesh>>,
    render_res: &mut ResMut<crate::rendering::SpriteResource>,
) {
    map_data.map = crate::procgen::make_map(level);

    // The actual map
    commands.spawn(PbrBundle {
        mesh: meshes.add( crate::modelgen::map_to_mesh(&map_data.map)),
        material: render_res.material.clone(),
        ..default()
    });

    // Player
    let player_pos = map_data.map.random_square();
    let player_pos = Vec3::new(player_pos.x as f32 + 0.5, 0.7, player_pos.z as f32 + 0.5);
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(player_pos).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(crate::player::PlayerBundle::default());
    map_data.player_pos = player_pos;

    for _ in 1 .. 20 {
        crate::ai::spawn_monster(commands, map_data, meshes, render_res);
    }
}