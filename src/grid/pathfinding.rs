use std::collections::VecDeque;

use super::{Coords, Grid};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Dir4 {
    #[default]
    None,
    End,
    N,
    E,
    S,
    W,
}

pub fn find_path4_to<T, F>(map: &Grid<T>, is_solid: F, end: Coords) -> (Grid<Dir4>, Grid<u32>)
where
    F: Fn(T) -> bool,
    T: Copy,
{
    let mut dirs = Grid::<Dir4>::new(map.x_max(), map.z_max());
    let mut distances = Grid::<u32>::new_from(map.x_max(), map.z_max(), u32::MAX);

    let mut test_queue = VecDeque::<(Coords, Dir4, u32)>::default();

    test_queue.push_back((end, Dir4::End, 0));

    while let Some((pos, dir, distance)) = test_queue.pop_front() {
        if dirs[pos] != Dir4::None || is_solid(map[pos]) {
            continue;
        }

        dirs[pos] = dir;
        distances[pos] = distance;
        test_queue.push_back((pos.right(), Dir4::W, distance + 1));
        test_queue.push_back((pos.left(), Dir4::E, distance + 1));
        test_queue.push_back((pos.bottom(), Dir4::N, distance + 1));
        test_queue.push_back((pos.top(), Dir4::S, distance + 1));
    }
    (dirs, distances)
}
