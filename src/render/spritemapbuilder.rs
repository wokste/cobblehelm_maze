use bevy::utils::default;

use bevy::{
    prelude::{AssetServer, Assets, Handle, Image, Res, ResMut},
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    utils::HashMap,
};

use super::spritemap::*;

trait Splitter {
    fn try_split(&mut self, scale: SpriteScale, count: USprite) -> Option<Self>
    where
        Self: Sized;
}

impl Splitter for SpriteSeq {
    fn try_split(&mut self, scale: SpriteScale, count: USprite) -> Option<SpriteSeq> {
        if self.scale != scale {
            return None;
        }

        let old_range = self.x.clone();
        let old_start = old_range.start;
        let old_len = old_range.end - old_range.start;

        if old_len < count {
            return None; // It doesn't fit
        }

        self.x.start += count;
        Some(SpriteSeq {
            x: old_start..(old_start + count),
            y: self.y,
            scale: self.scale,
        })
    }
}

#[derive(Debug)]
pub enum MapBuildError {
    IO,
    NoMoreRows,
    SequenceTooLong,
    BadSpriteHeight,
    BadSpriteRatio,
}

struct SpriteMapBuilder {
    sprites: SpriteMap,
    buckets: Vec<SpriteSeq>,
    next_row: u32,
    max_rows: u32,
}

impl SpriteMapBuilder {
    pub fn new(image: &Image) -> Self {
        let max_rows = (image.size().y as u32) / 64;

        Self {
            sprites: SpriteMap::default(),
            buckets: vec![],
            next_row: 0,
            max_rows,
        }
    }

    fn add_rows(&mut self, scale: SpriteScale) -> Result<&mut SpriteSeq, MapBuildError> {
        if self.next_row == self.max_rows {
            Err(MapBuildError::NoMoreRows)
        } else {
            let row = self.next_row;
            self.next_row += 1;

            let (x, y) = scale.row_capacity();
            for sub_row in 0..y {
                self.buckets.push(SpriteSeq {
                    scale,
                    x: 0..(x as USprite),
                    y: (row * y + sub_row) as USprite,
                });
            }

            let ret_pos = self.buckets.len() - y as usize;
            Ok(&mut self.buckets[ret_pos])
        }
    }

    fn find_sprites_pos(
        &mut self,
        scale: SpriteScale,
        count: USprite,
    ) -> Result<SpriteSeq, MapBuildError> {
        for bucket in self.buckets.iter_mut() {
            if let Some(seq) = bucket.try_split(scale, count) {
                return Ok(seq);
            }
        }

        // Nothing found. Will have to add a new sprite.
        let bucket = self.add_rows(scale)?;
        bucket
            .try_split(scale, count)
            .ok_or(MapBuildError::SequenceTooLong)
    }

    pub fn load_folder(
        &mut self,
        folder: &str,
        asset_server: &Res<AssetServer>,
        images: &mut ResMut<Assets<Image>>,
        dst_image: &mut Image,
    ) -> Result<HashMap<String, SpriteSeq>, MapBuildError> {
        let mut tiles = HashMap::<String, SpriteSeq>::new();

        let paths =
            std::fs::read_dir(format!("./assets/{}/", folder)).map_err(|e| MapBuildError::IO)?;
        for os_path in paths {
            let os_path = os_path.map_err(|e| MapBuildError::IO)?;

            let handle: Handle<Image> = asset_server.load(os_path.path());
            let src_image = images.get(&handle).unwrap();

            let (scale, count) = image_properties(src_image)?;

            let seq = self.find_sprites_pos(scale, count)?;
            copy_texture(src_image, dst_image, &seq);

            let key = os_path.file_name().to_str().unwrap().to_string();
            tiles.insert(key, seq);
        }
        Ok(tiles)
    }
}

fn image_properties(image: &Image) -> Result<(SpriteScale, u8), MapBuildError> {
    let w = image.size().x as u16;
    let h = image.size().y as u16;

    let scale = match h {
        64 => SpriteScale::Basic,
        32 => SpriteScale::Half,
        16 => SpriteScale::Quarter,
        _ => {
            return Err(MapBuildError::BadSpriteHeight);
        }
    };

    if w == 0 || (w % h) != 0 {
        return Err(MapBuildError::BadSpriteRatio);
    }
    Ok((scale, (w / h) as u8))
}

fn copy_texture(src: &Image, dest: &mut Image, pos: &SpriteSeq) {
    assert!(src.texture_descriptor.format == TextureFormat::Rgba8Uint);
    assert!(src.texture_descriptor.format == TextureFormat::Rgba8Uint);

    let px_mult = 4; // RGBA has 4 bytes
    let mult = pos.scale.size() as usize;
    let x0 = (pos.x.start as usize) * mult;

    let src_row = src.size().x as usize * px_mult;
    let dst_row = (TILESET_SIZE as usize) * px_mult;
    let y0 = (pos.y as usize) * mult;
    let y1 = ((pos.y + 1) as usize) * mult;

    for y in y0..y1 {
        let src_start = y * src_row;
        let dest_start = y * dst_row + x0;

        let src_slice = &src.data.as_slice()[src_start..src_start + src_row];
        let dest_slice = &mut dest.data.as_mut_slice()[dest_start..dest_start + src_row];

        dest_slice.copy_from_slice(src_slice);
    }
}

pub fn make_tilemap(
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
) -> Result<SpriteMap, MapBuildError> {
    let size = Extent3d {
        width: super::spritemap::TILESET_SIZE as u32,
        height: super::spritemap::TILESET_SIZE as u32,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Uint,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    let mut builder = SpriteMapBuilder::new(&image);

    builder.sprites.floors = builder.load_folder("floors", asset_server, images, &mut image)?;
    builder.sprites.walls = builder.load_folder("walls", asset_server, images, &mut image)?;
    builder.sprites.ceilings = builder.load_folder("ceilings", asset_server, images, &mut image)?;
    builder.sprites.monsters = builder.load_folder("monsters", asset_server, images, &mut image)?;
    builder.sprites.items = builder.load_folder("items", asset_server, images, &mut image)?;
    builder.sprites.projectiles =
        builder.load_folder("projectiles", asset_server, images, &mut image)?;
    builder.sprites.misc = builder.load_folder("misc", asset_server, images, &mut image)?;

    builder.sprites.texture = images.add(image);

    Ok(builder.sprites)
}
