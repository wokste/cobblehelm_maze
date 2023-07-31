use bevy::prelude::*;

use crate::{
    combat::{player::Player, CreatureStats},
    grid::Coords,
    physics::{MapCollisionEvent, PhysicsBody},
    rendering::{Sprite3d, SpriteResource},
    GameInfo,
};

#[derive(Component, Clone, Copy, PartialEq)]
pub enum Pickup {
    Apple,
    MedPack,
    NextLevel,
    Coin,
    CoinPile,
    SilverKey,
    GoldKey,
    RedKey,
    GreenKey,
}

enum StatGain {
    Health(i16),
    NextLevel,
    Coins(i32),
    Key(u8),
    //Ammo(u8,i16),
}

impl Pickup {
    const fn to_stat_gain(self) -> StatGain {
        match self {
            Pickup::Apple => StatGain::Health(15),
            Pickup::MedPack => StatGain::Health(45),
            Pickup::NextLevel => StatGain::NextLevel,
            Pickup::Coin => StatGain::Coins(1),
            Pickup::CoinPile => StatGain::Coins(5),
            Pickup::SilverKey => StatGain::Key(0x1),
            Pickup::GoldKey => StatGain::Key(0x2),
            Pickup::RedKey => StatGain::Key(0x4),
            Pickup::GreenKey => StatGain::Key(0x8),
        }
    }

    const fn can_take(self, stats: &CreatureStats) -> bool {
        match self.to_stat_gain() {
            StatGain::Health(_) => stats.hp < stats.hp_max,
            _ => true,
        }
    }

    fn take(
        &self,
        game_info: &mut GameInfo,
        stats: &mut Mut<CreatureStats>,
        game_state: &mut ResMut<NextState<crate::game::GameState>>,
    ) {
        match self.to_stat_gain() {
            StatGain::Health(gain) => {
                stats.hp = i16::min(stats.hp + gain, stats.hp_max);
                game_info.update_hp(stats);
            }
            StatGain::NextLevel => {
                game_state.set(crate::game::GameState::NextLevel);
            }
            StatGain::Coins(gain) => {
                game_info.coins += gain;
            }
            StatGain::Key(mask) => {
                game_info.key_flags |= mask;
            }
        }
        game_info.score += self.get_score(game_info.level as i32);
    }

    fn get_score(self, level: i32) -> i32 {
        match self.to_stat_gain() {
            StatGain::Health(_) => {
                if self == Pickup::MedPack {
                    -1000
                } else {
                    0
                }
            }
            StatGain::NextLevel => level * 250,
            StatGain::Coins(count) => count * 25,
            StatGain::Key(_) => level * 100,
        }
    }

    fn to_sound(self) -> Option<&'static str> {
        match self.to_stat_gain() {
            StatGain::Health(_) => Some("audio/pickup_heal.ogg"),
            StatGain::NextLevel => None,
            StatGain::Coins(_) => Some("audio/pickup_coins.ogg"),
            StatGain::Key(_) => Some("audio/pickup_key.ogg"),
        }
    }

    fn make_sprite(&self) -> Sprite3d {
        match self {
            Pickup::NextLevel => Sprite3d::basic(1, 5),

            Pickup::Coin => Sprite3d::half(6, 10),
            Pickup::CoinPile => Sprite3d::half(7, 10),
            Pickup::MedPack => Sprite3d::half(8, 10),
            Pickup::Apple => Sprite3d::half(9, 10),

            Pickup::SilverKey => Sprite3d::half(6, 11),
            Pickup::GoldKey => Sprite3d::half(7, 11),
            Pickup::RedKey => Sprite3d::half(8, 11),
            Pickup::GreenKey => Sprite3d::half(9, 11),
        }
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        map_data: &ResMut<crate::map::MapData>,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<SpriteResource>,
        rng: &mut fastrand::Rng,
    ) -> Result<(), &'static str> {
        let pos = choose_spawn_pos(map_data, rng)?;
        self.spawn_at_pos(pos, commands, meshes, render_res);
        Ok(())
    }

    pub fn spawn_at_pos(
        &self,
        pos: Coords,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<SpriteResource>,
    ) {
        let uv = self.make_sprite();

        commands
            .spawn(uv.to_sprite_bundle(pos.to_vec(0.25), meshes, render_res))
            .insert(crate::rendering::FaceCamera)
            .insert(*self)
            .insert(crate::physics::PhysicsBody::new(
                0.5,
                MapCollisionEvent::Stop,
            ));
    }
}

fn choose_spawn_pos(
    map_data: &crate::map::MapData,
    rng: &mut fastrand::Rng,
) -> Result<Coords, &'static str> {
    let solid_map = &map_data.solid_map;
    for _ in 0..4096 {
        let x = rng.i32(1..solid_map.x_max() - 1);
        let z = rng.i32(1..solid_map.z_max() - 1);

        if solid_map[(x, z)] {
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
    asset_server: Res<AssetServer>,
) {
    for (player_body, mut stats, player_transform) in player_query.iter_mut() {
        for (pickup_entity, pickup, pickup_body, pickup_transform) in pickup_query.iter_mut() {
            let distance = pickup_body.radius + player_body.radius;
            if pickup_transform
                .translation
                .distance_squared(player_transform.translation)
                > distance * distance
            {
                continue;
            }

            if pickup.can_take(&stats) {
                pickup.take(&mut game, &mut stats, &mut game_state);

                if let Some(filename) = pickup.to_sound() {
                    commands.spawn(AudioBundle {
                        source: asset_server.load(filename),
                        settings: default(),
                    });
                }

                commands.entity(pickup_entity).despawn();
            }
        }
    }
}
