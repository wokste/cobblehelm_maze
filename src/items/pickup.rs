use bevy::prelude::*;

use crate::{
    combat::{player::Player, CreatureStats},
    grid::Coords,
    physics::{MapCollisionEvent, PhysicsBody},
    render::{spritemap::USprite, RenderResource, Sprite3d},
    ui::menus::{MenuInfo, MenuType},
    GameInfo,
};

#[derive(Component, Clone, Copy, PartialEq)]
pub enum Pickup {
    Apple,
    MedPack,
    NextLevel,
    Coin,
    CoinPile,
    Key(u8),
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
            Pickup::Key(id) => StatGain::Key(1 << id),
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
        menu_info: &mut MenuInfo,
        stats: &mut Mut<CreatureStats>,
        game_state: &mut ResMut<NextState<crate::game::GameState>>,
    ) {
        match self.to_stat_gain() {
            StatGain::Health(gain) => {
                stats.hp = i16::min(stats.hp + gain, stats.hp_max);
            }
            StatGain::NextLevel => {
                game_state.set(crate::game::GameState::GameMenu);
                menu_info.set(MenuType::NextLevel);
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

    fn make_sprite(&self, tiles: &crate::render::spritemap::SpriteMap) -> Sprite3d {
        let (str, id) = match self {
            Pickup::Apple => ("apple.png", 0),
            Pickup::MedPack => ("medpack.png", 0),
            Pickup::NextLevel => ("portal.png", 0),
            Pickup::Coin => ("coin.png", 0),
            Pickup::CoinPile => ("gem.png", 0),
            Pickup::Key(id) => ("key.png", *id as USprite),
        };
        Sprite3d {
            tile: tiles.get_item(str).tile(id),
            flipped: false,
        }
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        map_data: &ResMut<crate::map::MapData>,
        meshes: &mut ResMut<Assets<Mesh>>,
        render_res: &mut ResMut<RenderResource>,
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
        render_res: &mut ResMut<RenderResource>,
    ) {
        let uv = self.make_sprite(&render_res.sprites);

        let size = uv.tile.scale.game_size();

        commands
            .spawn(uv.to_sprite_bundle(pos.to_vec(size * 0.5), meshes, render_res))
            .insert(crate::render::FaceCamera)
            .insert(*self)
            .insert(crate::physics::PhysicsBody::new(
                0.5, // TODO: Size
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
    mut menu_info: ResMut<MenuInfo>,
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
                pickup.take(&mut game, &mut menu_info, &mut stats, &mut game_state);

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
