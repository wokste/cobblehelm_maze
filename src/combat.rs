use bevy::prelude::Component;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Team {
    Players,
    Monsters,
//    Environment,
}

#[derive(Component)]
pub struct CreatureStats {
    pub speed: f32,
    pub hp: i16,
    pub hp_max: i16,
    pub team: Team,
}

impl CreatureStats {
    pub fn player() -> Self {
        Self {
            speed: 6.0,
            hp: 100,
            hp_max: 100,
            team: Team::Players,
        }
    }
}