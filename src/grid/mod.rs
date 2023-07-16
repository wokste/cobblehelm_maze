use std::ops::{Index, IndexMut};

mod coords;
mod rect;
mod transform;

pub use coords::Coords;
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
