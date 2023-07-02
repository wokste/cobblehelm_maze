use bevy::prelude::*;

use crate::{combat::{CreatureStats, player::Player}, GameInfo, physics::{PhysicsBody, MapCollisionEvent}, rendering::{SpriteResource, Sprite3d}, grid::Coords};

#[derive(Component, Clone, Copy)]
pub enum Pickup {
    Apple,
    MedPack,
    NextLevel,
    Coin,
}

enum StatGain {
    Health(i16),
    NextLevel,
    Coins(i32),
    //Key(u8),
    //Ammo(u8,i16),
}

impl Pickup {
    fn to_stat_gain(&self) -> StatGain {
        match self {
            Pickup::Apple => StatGain::Health(15),
            Pickup::MedPack => StatGain::Health(45),
            Pickup::NextLevel => StatGain::NextLevel,
            Pickup::Coin => StatGain::Coins(1),
        }
    }

    fn can_take(&self, stats: &CreatureStats) -> bool {
        match self.to_stat_gain() {
            StatGain::Health(_) => stats.hp < stats.hp_max,
            _=> true,
        }
    }

    fn take(&self, game_info: &mut GameInfo, stats: &mut Mut<CreatureStats>, game_state: &mut ResMut<NextState<crate::game::GameState>>) {
        match self.to_stat_gain() {
            StatGain::Health(gain) => {
                stats.hp = i16::min(stats.hp+gain, stats.hp_max);
            },
            StatGain::NextLevel => {
                game_state.set(crate::game::GameState::NextLevel);
            },
            StatGain::Coins(gain) => {
                game_info.coins += gain;
            },
        }
    }

    fn make_sprite(&self) -> Sprite3d {
        match self {
            Pickup::Apple => Sprite3d::half(6, 11),
            Pickup::MedPack => Sprite3d::half(7, 10),
            Pickup::NextLevel => Sprite3d::basic(1, 5),
            Pickup::Coin => Sprite3d::half(6, 10),
        }
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        map_data: &ResMut<crate::map::MapData>,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<SpriteResource>,
        rng: &mut fastrand::Rng,
    ) -> Result<(),&'static str> {
        let pos = choose_spawn_pos(map_data, rng)?;
        Ok(self.spawn_at_pos(pos, commands, meshes, render_res))
    }

    pub fn spawn_at_pos(
        &self,
        pos: Coords,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<SpriteResource>,
    ) {
        let uv = self.make_sprite();

        commands.spawn(uv.to_sprite_bundle(pos.to_vec(0.25), meshes, render_res))
            .insert(crate::rendering::FaceCamera)
            .insert(self.clone())
            .insert(crate::physics::PhysicsBody::new(0.5, MapCollisionEvent::Stop));
    }
}

fn choose_spawn_pos(map_data: &crate::map::MapData, rng: &mut fastrand::Rng) -> Result<Coords, &'static str> {
    
    let map = &map_data.map;
    for _ in 0 .. 4096 {
        let x = rng.i32(1 .. map.x_max() - 1);
        let z = rng.i32(1 .. map.z_max() - 1);

        if map[(x,z)].is_solid() {
            continue;
        }

        let pos = Coords::new(x, z);

        // TODO: Item check (Multiple items at the same spot)

        return Ok(pos);
    }
    Err("Could not find a proper item spawn pos")
}

pub fn check_pickups(
    mut commands: Commands,
    mut player_query: Query<(&PhysicsBody, &mut CreatureStats, &Transform), With<Player>>,
    mut pickup_query: Query<(Entity, &Pickup, &PhysicsBody, &Transform)>,
    mut game: ResMut<crate::GameInfo>,
    mut game_state: ResMut<NextState<crate::game::GameState>>,
) {
    for (player_body, mut stats, player_transform) in player_query.iter_mut() {
        for (pickup_entity, pickup, pickup_body, pickup_transform) in pickup_query.iter_mut() {
            let distance = pickup_body.radius + player_body.radius;
            if pickup_transform.translation.distance_squared(player_transform.translation) > distance * distance {
                continue;
            }

            if pickup.can_take(&stats) {
                pickup.take(&mut game, &mut stats, &mut game_state);
                commands.entity(pickup_entity).despawn();
            }
        }
    }
}