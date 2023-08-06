const TEX_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;

use bevy::{
    prelude::{AssetServer, Assets, Handle, Image, Res, ResMut},
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::TextureFormatPixelInfo,
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

        let path = format!("./assets/{}/", folder);
        let Ok(dir) = std::fs::read_dir(path) else {
            return Ok(tiles);
        };
        for os_path in dir {
            let os_path = os_path.map_err(|e| MapBuildError::IO)?;

            let handle: Handle<Image> = asset_server.load(os_path.path());
            let key = os_path.file_name().to_str().unwrap().to_string();
            println!("found sprite {}", key);
            let src_image = images.get(&handle).unwrap();

            let (scale, count) = image_properties(src_image)?;

            let seq = self.find_sprites_pos(scale, count)?;
            copy_texture(src_image, dst_image, &seq);

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
    assert!(src.texture_descriptor.format == TEX_FORMAT);
    assert!(src.texture_descriptor.format == TEX_FORMAT);

    let px_mult = TEX_FORMAT.pixel_size(); // RGBA has 4 bytes
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
    let width = super::spritemap::TILESET_SIZE as u32;
    let height = super::spritemap::TILESET_SIZE as u32;

    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        vec![0; TEX_FORMAT.pixel_size() * (width * height) as usize],
        TEX_FORMAT,
    );

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
    builder.sprites.no_tile = SpriteSeq {
        x: 0..1,
        y: 0,
        scale: SpriteScale::Basic,
    };
    builder.sprites.no_tile = builder.sprites.get_misc("missing.png");

    Ok(builder.sprites)
}
