use crate::{combat::MonsterType, map::DoorType, mapgen::style::LevelStyle};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpawnObject {
    Portal {
        style: LevelStyle,
    },
    #[allow(dead_code)] // TODO: Remove after 0.2
    Monster {
        monster_type: MonsterType,
    },
    Door {
        door_type: DoorType,
        is_vertical: bool,
    },
    Shop,
    Phylactery,
}
