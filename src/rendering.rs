use bevy::prelude::{Component, Vec2};



#[derive(Clone)]
pub struct TexCoords {
    pub x : std::ops::Range<u8>,
    pub y : u8,
}

impl TexCoords {
    pub fn new(x : std::ops::Range<u8>, y : u8) -> Self {
        Self{x,y}
    }

    pub fn to_uv(&self) -> Vec2 {
        let x = fastrand::u8(self.x.clone());
        let y = self.y;

        Vec2::new(x as f32 / 32.0, y as f32 / 8.0)
    }
}


#[derive(Component, Default)]
pub struct Sprite;