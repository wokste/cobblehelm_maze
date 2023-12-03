use std::ops::{Index, IndexMut};

mod coords;
mod pathfinding;
mod rect;
mod transform;

pub use coords::Coords;
pub use pathfinding::*;
pub use rect::Rect;
pub use transform::GridTransform;

pub struct Grid<T> {
    tiles: Vec<T>,
    size: Coords,
}

impl<T> Grid<T>
where
    T: Default,
    T: Clone,
{
    pub fn new(x_max: i32, z_max: i32) -> Self {
        Self {
            tiles: vec![T::default(); (x_max * z_max) as usize],
            size: Coords::new(x_max, z_max),
        }
    }

    pub fn new_from(x_max: i32, z_max: i32, val: T) -> Self {
        Self {
            tiles: vec![val; (x_max * z_max) as usize],
            size: Coords::new(x_max, z_max),
        }
    }
}

impl<T> Grid<T> {
    pub fn x_max(&self) -> i32 {
        self.size.x
    }
    pub fn z_max(&self) -> i32 {
        self.size.z
    }
    pub fn size(&self) -> Rect {
        Rect {
            p0: Coords::ZERO,
            p1: self.size,
        }
    }

    pub fn contains_coord(&self, x: i32, z: i32) -> bool {
        x >= 0 && x < self.size.x && z >= 0 && z < self.size.z
    }

    fn to_index(&self, x: i32, z: i32) -> usize {
        assert!(
            self.contains_coord(x, z),
            "Error: ({},{}) not in range (0..{},0..{})",
            x,
            z,
            self.size.x,
            self.size.z
        );

        (x + z * self.size.x) as usize
    }

    fn to_coord(size: Coords, index: usize) -> Coords {
        assert!(
            index < (size.x * size.z) as usize,
            "Error: {} not in range (0..{}*{})",
            index,
            size.x,
            size.z,
        );

        let index = index as i32;
        let x = index % size.z;
        let z = index / size.z;

        Coords::new(x, z)
    }

    pub fn map<B, F>(&self, f: F) -> Grid<B>
    where
        F: FnMut(T) -> B,
        B: Sized,
        T: Clone,
    {
        Grid::<B> {
            size: self.size,
            tiles: self.tiles.clone().into_iter().map(f).collect(),
        }
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn iter(&self) -> impl Iterator<Item = (Coords, T)> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| (Self::to_coord(self.size, index), *tile))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Coords, &mut T)> + '_ {
        self.tiles
            .iter_mut()
            .enumerate()
            .map(|(index, tile)| (Self::to_coord(self.size, index), tile))
    }
}

impl<T> Index<Coords> for Grid<T> {
    type Output = T;

    fn index(&self, c: Coords) -> &Self::Output {
        &self.tiles[self.to_index(c.x, c.z)]
    }
}

impl<T> IndexMut<Coords> for Grid<T> {
    fn index_mut(&mut self, c: Coords) -> &mut Self::Output {
        let id: usize = self.to_index(c.x, c.z);
        &mut self.tiles[id]
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    fn index(&self, c: (i32, i32)) -> &Self::Output {
        &self.tiles[self.to_index(c.0, c.1)]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, c: (i32, i32)) -> &mut Self::Output {
        let id: usize = self.to_index(c.0, c.1);
        &mut self.tiles[id]
    }
}

// TODO: Unittest for to_coords() and to_index();
