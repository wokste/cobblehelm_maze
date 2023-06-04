use crate::map::Coords;

#[derive(Copy, Clone, PartialEq)]
pub struct MapTransform{
    dx: i32,
    dz: i32,
    flip_x: bool,
    flip_z: bool,
    swap_xz: bool,
}

impl MapTransform {
    pub fn make_rand(map_size: Coords, room_size: Coords) -> MapTransform {
        let swap_xz = fastrand::bool();

        let room_size = if swap_xz {room_size.transpose() } else {room_size};

        let mut transform = MapTransform::new(
            fastrand::i32(0 .. map_size.x - room_size.x),
            fastrand::i32(0 .. map_size.z - room_size.z),
            swap_xz,
        );

        if fastrand::bool() {transform.do_flip_x(room_size);}
        if fastrand::bool() {transform.do_flip_z(room_size);}

        transform
    }

    pub fn new(dx: i32, dz:i32, swap_xz : bool) -> MapTransform {
        MapTransform{ dx, dz, flip_x : false, flip_z : false, swap_xz, }
    }

    fn do_flip_x(&mut self, roomsize : Coords) {
        let delta = roomsize.x - 1;
        self.flip_x = !self.flip_x;
        if self.flip_x {self.dx += delta;} else {self.dx -= delta;}
    }
    
    fn do_flip_z(&mut self, roomsize : Coords) {
        let delta = roomsize.z - 1;
        self.flip_z = !self.flip_z;
        if self.flip_z {self.dz += delta;} else {self.dz -= delta;}
    }

    pub fn map_xz(&self, x : i32, z : i32) -> Coords {
        self.map(Coords::new(x,z))
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
        let mut t = MapTransform::new(4,8,false);
        assert_eq!(t.map_xz(1,3), Coords::new(5,11));

        t.do_flip_x(Coords::new(8,8));
        assert_eq!(t.map_xz(6,3), Coords::new(5,11));

        t.do_flip_z(Coords::new(8,8));
        assert_eq!(t.map_xz(6,4), Coords::new(5,11));
    }

    #[test]
    fn map_transpose() {
        let mut t = MapTransform::new(4,8,true);
        assert_eq!(t.map_xz(3,1), Coords::new(5,11));

        t.do_flip_z(Coords::new(8,8));
        assert_eq!(t.map_xz(4,1), Coords::new(5,11));

        t.do_flip_x(Coords::new(8,8));
        assert_eq!(t.map_xz(4,6), Coords::new(5,11));
    }
}