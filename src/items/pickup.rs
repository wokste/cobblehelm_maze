use bevy::prelude::*;

use crate::{
    combat::{player::Player, CreatureStats},
    physics::PhysicsBody,
    render::{spritemap::USprite, Sprite3d},
    ui::menus::{MenuInfo, MenuType},
    GameInfo,
};

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum Pickup {
    Apple,
    MedPack,
    Coin,
    Gem,
    Phylactery,
    Key(u8),
}

enum StatGain {
    PercHealth(i16),
    Coins(i32),
    Phylactery,
    Key(u8),
    //Ammo(u8,i16),
}

impl Pickup {
    const fn to_stat_gain(self) -> StatGain {
        match self {
            Pickup::Apple => StatGain::PercHealth(20),
            Pickup::MedPack => StatGain::PercHealth(50),
            Pickup::Coin => StatGain::Coins(1),
            Pickup::Gem => StatGain::Coins(5),
            Pickup::Key(id) => StatGain::Key(1 << id),
            Pickup::Phylactery => StatGain::Phylactery,
        }
    }

    const fn can_take(self, stats: &CreatureStats) -> bool {
        match self.to_stat_gain() {
            StatGain::PercHealth(_) => stats.hp < stats.hp_max,
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
            StatGain::PercHealth(perc) => {
                let gain = (stats.hp_max * perc + 50) / 100;
                stats.hp = i16::min(stats.hp + gain, stats.hp_max);
            }
            StatGain::Coins(gain) => {
                game_info.coins += gain;
            }
            StatGain::Key(mask) => {
                game_info.key_flags |= mask;
            }
            StatGain::Phylactery => {
                game_state.set(crate::game::GameState::GameMenu);
                menu_info.set(MenuType::Victory);
            }
        }
        game_info.score += self.get_score(game_info.level as i32);
    }

    fn get_score(self, level: i32) -> i32 {
        match self.to_stat_gain() {
            StatGain::PercHealth(_) => 0,
            StatGain::Coins(count) => count * 25,
            StatGain::Key(_) => level * 100,
            StatGain::Phylactery => 5000,
        }
    }

    fn to_sound(self) -> Option<&'static str> {
        match self.to_stat_gain() {
            StatGain::PercHealth(_) => Some("audio/pickup_heal.ogg"),
            StatGain::Coins(_) => Some("audio/pickup_coins.ogg"),
            StatGain::Key(_) => Some("audio/pickup_key.ogg"),
            StatGain::Phylactery => Some("audio/phylactery.ogg"),
        }
    }

    pub fn make_sprite(&self, tiles: &crate::render::spritemap::SpriteMap) -> Sprite3d {
        let (str, id) = match self {
            Pickup::Apple => ("apple.png", 0),
            Pickup::MedPack => ("medpack.png", 0),
            Pickup::Coin => ("coin.png", 0),
            Pickup::Gem => ("gem.png", 0),
            Pickup::Key(id) => ("key.png", *id as USprite),
            Pickup::Phylactery => ("phylactery.png", 0),
        };
        Sprite3d {
            tile: tiles.get_item(str).tile(id),
            flipped: false,
            two_sided: false,
        }
    }
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
