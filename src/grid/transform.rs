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

        println!("Created transform:{:?}, map_size:{:?} room_size:{:?}", transform, map_size, room_size);
        println!("Mapped this is {:?}=>{:?}, {:?}=>{:?}", room_size.p0, transform.map(room_size.p0), room_size.p1, transform.map(room_size.p1));
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

    pub fn map_xz(&self, x: i32, z: i32) -> Coords {
        self.map(Coords::new(x,z))
    }

    pub fn map_rect(&self, r: Rect) -> Rect {
        Rect::from_pairs(self.map(r.p0), self.map(r.p1))
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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn map_simple() {
        let mut t = GridTransform::new(4,8,false);
        assert_eq!(t.map_xz(1,3), Coords::new(5,11));

        t.do_flip_x(Rect::from_xz(8,8));
        assert_eq!(t.map_xz(6,3), Coords::new(5,11));

        t.do_flip_z(Rect::from_xz(8,8));
        assert_eq!(t.map_xz(6,4), Coords::new(5,11));
    }

    #[test]
    fn map_transpose() {
        let mut t = GridTransform::new(4,8,true);
        assert_eq!(t.map_xz(3,1), Coords::new(5,11));

        t.do_flip_z(Rect::from_xz(8,8));
        assert_eq!(t.map_xz(4,1), Coords::new(5,11));

        t.do_flip_x(Rect::from_xz(8,8));
        assert_eq!(t.map_xz(4,6), Coords::new(5,11));
    }
}