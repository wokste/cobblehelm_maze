use std::fmt;

use super::Coords;


#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Rect {
    pub p0 : Coords,
    pub p1 : Coords,
}

impl std::fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}..{:?}]", self.p0, self.p1)
    }
}

impl Rect {
    pub fn rand_center(&self, rng: &mut fastrand::Rng) -> Coords {
        Coords::new(
            ((self.p1.x - self.p0.x) + rng.bool() as i32) / 2 + self.p0.x,
            ((self.p1.z - self.p0.z) + rng.bool() as i32) / 2 + self.p0.z
        )
    }

    pub fn rand(&self, rng: &mut fastrand::Rng) -> Coords {
        Coords::new(
            rng.i32(self.p0.x .. self.p1.x),
            rng.i32(self.p0.z .. self.p1.z),
        )
    }

    pub const fn transpose(self) -> Self {
        Self {
            p0: self.p0.transpose(),
            p1: self.p1.transpose(),
        }
    }

    pub fn shrink(self, delta: i32) -> Self {
        let delta = Coords::new(delta, delta);
        Self {
            p0: self.p0 + delta,
            p1: self.p1 - delta,
        }
    }

    pub fn iter(&self) -> RectIter {
        RectIter{
            pos: self.p0,
            rect: self,
        }
    }
}

pub struct RectIter<'a>{
    pos : Coords,
    rect : &'a Rect
}

impl<'a> Iterator for RectIter<'a> {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        let old_pos = self.pos;
        
        if old_pos.x < self.rect.p1.x {
            self.pos.z += 1;
            if self.pos.z >= self.rect.p1.z {
                self.pos.z = self.rect.p0.z;
                self.pos.x += 1;
            }

            Some(old_pos)
        } else {
            None
        }
    }
}