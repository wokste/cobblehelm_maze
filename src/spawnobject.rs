use crate::{
    combat::MonsterType,
    grid::{Coords, Grid},
    map::{DoorType, Tile},
    mapgen::style::LevelStyle,
};

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

impl SpawnObject {
    pub fn validate_pos(&self, pos: Coords, grid: &Grid<Tile>) -> bool {
        match self {
            SpawnObject::Door { is_vertical, .. } => {
                // TODO: Check order
                if *is_vertical {
                    grid[pos.top()].is_solid() && grid[pos.bottom()].is_solid()
                } else {
                    grid[pos.left()].is_solid() && grid[pos.right()].is_solid()
                }
            }
            SpawnObject::Shop => true,
            _ => true,
        }
    }
}
