use crate::{combat::MonsterType, map::DoorType, mapgen::style::LevelStyle};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpawnObject {
    Portal { style: LevelStyle },
    Monster { monster_type: MonsterType },
    Door { door_type: DoorType, vertical: bool },
    Phylactery,
}
