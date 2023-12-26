use crate::{
    grid::{Coords, Grid},
    mapgen::randitem::RandItem,
    spawnobject::SpawnObject,
};

pub fn add_level_transition_objects(
    dist_map: &Grid<u32>,
    rng: &mut fastrand::Rng,
    spawn_objects: &mut Vec<(Coords, SpawnObject)>,
    level: u8,
) {
    let items_to_spawn = choose_level_transition_items(rng, level);
    spawn_object_instances(dist_map, rng, spawn_objects, items_to_spawn)
}

fn choose_level_transition_items(rng: &mut fastrand::Rng, level: u8) -> Vec<SpawnObject> {
    use crate::mapgen::style::{ALT_LEVELS, BASE_LEVELS};
    use SpawnObject as SO;

    let style_index = (level - 1) as usize + 1;

    if style_index < BASE_LEVELS.len() {
        let base_style = BASE_LEVELS[style_index];
        let mut portals = vec![SO::Portal { style: base_style }];

        // The last level is always the same and has no choice
        // Otherwise make a second portal to the last level
        if style_index < BASE_LEVELS.len() - 1 {
            let alt_style_index =
                (level as i32 + rng.i32(-1..=1)).clamp(0, BASE_LEVELS.len() as i32 - 1);
            let mut alt_style = BASE_LEVELS[alt_style_index as usize];

            if base_style == alt_style {
                alt_style = *ALT_LEVELS.rand_front_loaded(rng);
            }

            portals.push(SO::Portal { style: alt_style })
        }

        portals
    } else {
        // In the endboss level, add a phylactery and the lich.
        vec![SpawnObject::Phylactery]

        // TODO: Add lich
    }
}

fn spawn_object_instances(
    dist_map: &Grid<u32>,
    rng: &mut fastrand::Rng,
    spawn_objects: &mut Vec<(Coords, SpawnObject)>,
    objects_to_spawn: Vec<SpawnObject>,
) {
    let mut positions: Vec<_> = dist_map
        .iter()
        .filter(|(_, dist)| *dist != u32::MAX)
        .collect();

    let Some((_, max_dist)) = positions.iter().max_by_key(|(_, d)| d) else {
        panic!("Can't spawn objects");
        // TODO: Maybe not panic
    };

    let required_dist = max_dist * 8 / 10;
    positions.retain(|(_, dist)| *dist >= required_dist);

    for obj in objects_to_spawn.iter() {
        let index = rng.usize(0..positions.len());
        spawn_objects.push((positions[index].0, *obj));
        positions.swap_remove(index);
    }
}
