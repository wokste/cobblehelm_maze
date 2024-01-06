use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::PbrBundle,
    render::mesh::Mesh,
    transform::components::Transform,
};

use crate::{
    combat::{ai::AiMover, MonsterType},
    grid::Coords,
    interactable::{Door, Interactable},
    items::pickup::Pickup,
    physics::{Collider, MapCollisionEvent},
    render::{FaceCamera, Sprite3d},
    spawnobject::SpawnObject,
};

pub struct Spawner<'c1, 'c2, 'ma, 'me, 'r> {
    pub commands: Commands<'c1, 'c2>,
    pub map_data: ResMut<'ma, crate::map::MapData>,
    pub meshes: ResMut<'me, Assets<Mesh>>,
    pub render_res: ResMut<'r, crate::render::RenderResource>,
}

impl Spawner<'_, '_, '_, '_, '_> {
    // ---- ITEMS ----
    pub fn try_spawn_item(&mut self, item: Pickup, rng: &mut fastrand::Rng) -> bool {
        match self.choose_item_pos(rng) {
            Ok(pos) => {
                self.spawn_item_at_pos(pos, item);
                true
            }
            Err(err) => {
                println!("Failed to spawn item {:?}: {}", item, err);
                false
            }
        }
    }

    pub fn spawn_item_at_pos(&mut self, pos: Coords, item: Pickup) {
        let uv = item.make_sprite(&self.render_res.sprites);

        let size = uv.tile.scale.game_size();
        let pos = pos.to_vec(size * 0.5);

        self.commands
            .spawn(uv.to_sprite_bundle(pos, &mut self.meshes, &mut self.render_res))
            .insert(crate::render::FaceCamera)
            .insert(item)
            .insert(crate::physics::Collider::new(
                pos,
                0.5, // TODO: Size
                MapCollisionEvent::Stop,
            ));
    }

    pub fn choose_item_pos(&mut self, rng: &mut fastrand::Rng) -> Result<Coords, &'static str> {
        let solid_map = &self.map_data.solid_map;
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

    // -- MONSTERS ---

    pub fn try_spawn_monster(&mut self, monster: MonsterType, rng: &mut fastrand::Rng) -> bool {
        let Ok(pos) = self.choose_monster_pos(rng) else {
            println!("Failed top spawn monster: {:?}", monster);
            return false;
        };

        self.spawn_monster_at_pos(pos, monster, rng);
        true
    }

    pub fn spawn_monster_at_pos(
        &mut self,
        pos: Coords,
        monster: MonsterType,
        rng: &mut fastrand::Rng,
    ) {
        let mover = AiMover::new(pos, &mut self.map_data.monster_map);
        let pos = mover.to_vec(monster.jumps(), 0.0);
        let uv = monster.get_tile_seq(&self.render_res.sprites);

        self.commands
            .spawn(uv.to_sprite_bundle(pos, &mut self.meshes, &mut self.render_res))
            .insert(crate::render::Animation::new(uv, rng.f32() * 0.04 + 0.16))
            .insert(monster.make_ai())
            .insert(mover)
            .insert(monster.make_stats())
            .insert(monster.make_weapon())
            .insert(crate::physics::Collider::new(
                pos,
                0.5,
                MapCollisionEvent::Stop,
            ));
    }

    pub fn choose_monster_pos(&mut self, rng: &mut fastrand::Rng) -> Result<Coords, &'static str> {
        for _ in 0..4096 {
            let pos = self.map_data.solid_map.size().shrink(1).rand(rng);

            if self.map_data.solid_map[pos] || self.map_data.monster_map[pos] {
                continue;
            }

            if self.map_data.can_see_player(pos.to_vec(0.5), 15.0) {
                continue;
            }

            return Ok(pos);
        }
        Err("Could not find a proper spawn pos")
    }

    // --- Objects ---
    pub fn spawn_object_at_pos(
        &mut self,
        pos: Coords,
        object_type: &SpawnObject,
        rng: &mut fastrand::Rng,
    ) {
        //let size = uv.tile.scale.game_size();

        match object_type {
            SpawnObject::Portal { style } => {
                // TODO: get_portal?
                let uv = &self.render_res.sprites.get_item(style.portal_sprite());
                let sprite = Sprite3d::new(uv.tile_start());

                let transform =
                    Transform::from_translation(pos.to_vec(0.5)).looking_to(Vec3::X, Vec3::Y);

                self.commands
                    .spawn(PbrBundle {
                        mesh: self.render_res.get_mesh(sprite, &mut self.meshes),
                        material: self.render_res.material.clone(),
                        transform,
                        ..Default::default()
                    })
                    .insert(crate::lifecycle::LevelObject)
                    .insert(FaceCamera)
                    .insert(Interactable::NextLevel(*style))
                    .insert(Collider::new(pos.to_vec(0.5), 0.5, MapCollisionEvent::Stop))
                    .insert(sprite);
            }
            SpawnObject::Monster { monster_type } => {
                self.spawn_monster_at_pos(pos, *monster_type, rng)
            }
            SpawnObject::Door {
                door_type,
                is_vertical,
            } => {
                let uv = door_type.make_sprite(&self.render_res.sprites);
                let door = Door::new(*door_type, uv, *is_vertical);

                door.update_collision(pos, &mut self.map_data);

                let mut direction = if *is_vertical { Vec3::X } else { Vec3::Z };

                // Flip half the doors, just to add a bit of randomization
                if rng.bool() {
                    direction = -direction;
                };

                let transform =
                    Transform::from_translation(pos.to_vec(0.5)).looking_to(direction, Vec3::Y);

                self.commands
                    .spawn(PbrBundle {
                        mesh: self
                            .render_res
                            .get_mesh(door.make_sprite3d(), &mut self.meshes),
                        material: self.render_res.material.clone(),
                        transform,
                        ..Default::default()
                    })
                    .insert(crate::lifecycle::LevelObject)
                    .insert(Interactable::SelfTrigger)
                    .insert(Collider::new(pos.to_vec(0.5), 0.5, MapCollisionEvent::Stop))
                    .insert(door.make_sprite3d())
                    .insert(door);
            }
            SpawnObject::Shop => {
                let uv = &self.render_res.sprites.misc["vending_machine.png"];
                let sprite = Sprite3d::new(uv.tile_start()).make_two_sided();

                // TODO: Choose dir
                let transform =
                    Transform::from_translation(pos.to_vec(0.5)).looking_to(Vec3::X, Vec3::Y);

                self.commands
                    .spawn(PbrBundle {
                        mesh: self.render_res.get_mesh(sprite, &mut self.meshes),
                        material: self.render_res.material.clone(),
                        transform,
                        ..Default::default()
                    })
                    .insert(crate::lifecycle::LevelObject)
                    .insert(Interactable::Shop)
                    .insert(Collider::new(pos.to_vec(0.5), 0.5, MapCollisionEvent::Stop))
                    .insert(FaceCamera)
                    .insert(sprite);
            }
            SpawnObject::Phylactery {} => self.spawn_item_at_pos(pos, Pickup::Phylactery),
        }
    }
}
