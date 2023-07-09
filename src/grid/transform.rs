use crate::grid::{Coords, Rect};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GridTransform{
    dx: i32,
    dz: i32,
    flip_x: bool,
    flip_z: bool,
    swap_xz: bool,
}

impl GridTransform {
    pub fn make_rand(map_size: Rect, room_size: Rect, rng: &mut fastrand::Rng) -> GridTransform {
        let swap_xz = rng.bool();

        let room_size = if swap_xz {room_size.transpose() } else {room_size};

        // TODO: Use x0
        let mut transform = GridTransform::new(
            rng.i32(0 .. map_size.p1.x - room_size.p1.x),
            rng.i32(0 .. map_size.p1.z - room_size.p1.z),
            swap_xz,
        );

        if rng.bool() {transform.do_flip_x(room_size);}
        if rng.bool() {transform.do_flip_z(room_size);}

        transform
    }

    pub fn new(dx: i32, dz:i32, swap_xz: bool) -> GridTransform {
        GridTransform{ dx, dz, flip_x: false, flip_z: false, swap_xz, }
    }

    fn do_flip_x(&mut self, roomsize: Rect) {
        let delta = roomsize.p1.x - 1; // TODO: x0
        self.flip_x = !self.flip_x;
        if self.flip_x {self.dx += delta;} else {self.dx -= delta;}
    }
    
    fn do_flip_z(&mut self, roomsize: Rect) {
        let delta = roomsize.p1.z - 1; // TODO: z0
        self.flip_z = !self.flip_z;
        if self.flip_z {self.dz += delta;} else {self.dz -= delta;}
    }

    pub fn map(&self, coords: Coords) -> Coords {
        let mut coords = coords;

        if self.swap_xz { coords = Coords::new(coords.z, coords.x); }

        if self.flip_x { coords.x *= -1; }
        if self.flip_z { coords.z *= -1; }

        coords.x += self.dx;
        coords.z += self.dz;

        coords
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn coord(x:i32, z:i32) -> Coords { Coords::new(x,z) }

    const ROOM_SIZE : Rect = Rect{p0: Coords::ZERO, p1: coord(8,8)};

    #[test]
    fn map_simple() {
        let mut t = GridTransform::new(4,8,false);
        assert_eq!(t.map(coord(1,3)), coord(5,11));

        t.do_flip_x(ROOM_SIZE);
        assert_eq!(t.map(coord(6,3)), coord(5,11));

        t.do_flip_z(ROOM_SIZE);
        assert_eq!(t.map(coord(6,4)), coord(5,11));
    }

    #[test]
    fn map_transpose() {
        let mut t = GridTransform::new(4,8,true);
        assert_eq!(t.map(coord(3,1)), coord(5,11));

        t.do_flip_z(ROOM_SIZE);
        assert_eq!(t.map(coord(4,1)), coord(5,11));

        t.do_flip_x(ROOM_SIZE);
        assert_eq!(t.map(coord(4,6)), coord(5,11));
    }
}