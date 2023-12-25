use bevy::{
    asset::{Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        system::{Query, Res, ResMut},
    },
    render::mesh::Mesh,
    transform::components::Transform,
};

use crate::{
    grid::Coords,
    map::{DoorType, MapData},
    render::{spritemap::SpriteSeq, RenderResource, Sprite3d},
    GameInfo,
};

#[derive(Component)]
pub enum Interactable {
    SelfTrigger,
    #[allow(dead_code)] // TODO: Remove after 0.2
    Trigger(Vec<Entity>),
    Shop,
}

impl Interactable {
    pub fn in_range(player: &Transform, interactable: &Transform) -> bool {
        const ARM_LENGTH: f32 = 0.7;
        const RADIUS: f32 = 0.7;

        let hand_pos = player.translation + player.forward() * ARM_LENGTH;
        // TODO: Cache look_pos

        hand_pos.distance_squared(interactable.translation) < RADIUS * RADIUS
    }
}

#[derive(Event, Clone, Copy, PartialEq)]
pub struct TriggerEvent {
    pub target: Entity,
    pub instigator: Option<Entity>,
}

#[derive(Component)]
pub struct Door {
    pub door_type: DoorType,
    pub sprites: SpriteSeq,
    pub is_open: bool,
    pub is_vertical: bool,
    pub required_key: u8,
}

impl Door {
    pub fn new(door_type: DoorType, sprites: SpriteSeq, is_vertical: bool) -> Self {
        Self {
            door_type,
            sprites,
            is_open: false,
            is_vertical,
            required_key: 0,
        }
    }

    pub fn update_collision(&self, pos: Coords, map: &mut ResMut<MapData>) {
        let collision = !self.is_open;
        map.solid_map[pos] = collision;
        map.los_map[pos] = collision;
    }

    pub fn sprite(&self) -> crate::render::spritemap::SpritePos {
        let id = match (self.door_type, self.is_open) {
            (_, true) => 1,
            (_, false) => 0,
        };
        self.sprites.tile(id)
    }

    pub fn make_sprite3d(&self) -> Sprite3d {
        Sprite3d {
            tile: self.sprite(),
            flipped: false,
            two_sided: true,
        }
    }
}

pub fn update_doors(
    mut events: EventReader<TriggerEvent>,
    mut door_query: Query<(&mut Door, &mut Sprite3d, &Transform, &mut Handle<Mesh>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_res: ResMut<RenderResource>,
    game_info: Res<GameInfo>,

    mut map: ResMut<MapData>,
) {
    for event in events.read() {
        if let Ok((mut door, mut sprite, transform, mut mesh)) = door_query.get_mut(event.target) {
            //if door.is_open {
            //    continue;
            //}

            // Locked doors
            if (door.required_key & !game_info.key_flags) != 0 {
                // Door cannot be opened, because it is locked

                // TODO: Play nope sound
                continue;
            }

            door.is_open = !door.is_open;

            sprite.tile = door.sprite();
            *mesh = render_res.get_mesh(*sprite, &mut meshes);

            let pos = Coords::from_vec(transform.translation);

            door.update_collision(pos, &mut map);
        }
    }
}
