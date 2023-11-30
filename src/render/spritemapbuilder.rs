const TEX_FORMAT: TextureFormat = TextureFormat::Rgba8UnormSrgb;

use std::path::{Path, PathBuf};

use bevy::{
    prelude::{warn, AssetServer, Assets, Handle, Image, Res, ResMut, Resource},
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::TextureFormatPixelInfo,
    },
};

use super::spritemap::*;

trait Splitable {
    fn try_split(&mut self, scale: SpriteScale, count: USprite) -> Option<Self>
    where
        Self: Sized;
}

impl Splitable for SpriteSeq {
    fn try_split(&mut self, scale: SpriteScale, count: USprite) -> Option<SpriteSeq> {
        assert!(count > 0);

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
    IO(std::io::Error),
    NoMoreRows,
    SequenceTooLong,
    BadSpriteHeight(u16),
    BadSpriteRatio,
}

#[derive(Resource)]
pub struct SpriteMapBuilder {
    loaded: bool,
    loading: Vec<(String, Handle<Image>, SpriteGroup)>,
    buckets: Vec<SpriteSeq>,
    next_row: u32,
    max_rows: u32,
}

impl SpriteMapBuilder {
    pub fn new() -> Self {
        const HEIGHT: u32 = super::spritemap::TILESET_SIZE as u32;
        let max_rows = HEIGHT / 64;

        Self {
            loaded: false,
            loading: vec![],
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
        group: SpriteGroup,
        asset_server: &Res<AssetServer>,
    ) -> Result<(), MapBuildError> {
        let path = format!("./assets/{}/", folder);
        let Ok(dir) = std::fs::read_dir(path) else {
            return Ok(());
        };
        for os_path in dir {
            let os_path = os_path.map_err(|e| MapBuildError::IO(e))?;
            let key = os_path.file_name().to_str().unwrap().to_string();
            let os_path: PathBuf = Path::new(".").join(folder).join(os_path.file_name());

            println!("Loading image: {:?}", os_path);
            let handle: Handle<Image> = asset_server.load(os_path);

            self.loading.push((key, handle, group));
        }
        Ok(())
    }

    pub fn should_build(&self, images: &ResMut<Assets<Image>>) -> bool {
        if self.loaded == true {
            return false;
        }

        for (_str, handle, _) in self.loading.iter() {
            if !images.contains(handle) {
                return false;
            }
        }
        true
    }

    pub fn build_done(&self) -> bool {
        self.loaded
    }

    pub fn build(
        &mut self,
        images: &mut ResMut<Assets<Image>>,
    ) -> Result<SpriteMap, MapBuildError> {
        assert!(self.should_build(images));
        println!("Build started");
        // Create the texture map image
        let width = super::spritemap::TILESET_SIZE as u32;
        let height = super::spritemap::TILESET_SIZE as u32;
        let mut dst_image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            vec![0; TEX_FORMAT.pixel_size() * (width * height) as usize],
            TEX_FORMAT,
        );

        // Copy the loaded images to said image
        let mut map = SpriteMap::default();

        let loaded: Vec<_> = self.loading.drain(0..self.loading.len()).collect();
        for (key, handle, group) in loaded {
            let src_image = images.get(&handle).unwrap();

            let (scale, count) = image_properties(src_image)?;
            let seq = self.find_sprites_pos(scale, count)?;
            copy_texture(src_image, &mut dst_image, &seq);

            map.find_map_mut(group).insert(key, seq);
        }

        // Bind image
        map.texture = images.add(dst_image);

        self.loaded = true;
        println!("Build done");

        // Return it
        Ok(map)
    }

    pub fn start_load(&mut self, asset_server: &Res<AssetServer>) -> Result<(), MapBuildError> {
        type SG = super::spritemap::SpriteGroup;
        self.load_folder("blocks", SG::Block, asset_server)?;
        self.load_folder("monsters", SG::Monster, asset_server)?;
        self.load_folder("items", SG::Item, asset_server)?;
        self.load_folder("projectiles", SG::Projectile, asset_server)?;
        self.load_folder("misc", SG::Misc, asset_server)?;
        Ok(())
    }
}

fn image_properties(image: &Image) -> Result<(SpriteScale, u8), MapBuildError> {
    let w = image.size().x as u16;
    let h = image.size().y as u16;
    let label = image.texture_descriptor.label;

    let scale = match h {
        64 => SpriteScale::Basic,
        32 => SpriteScale::Half,
        16 => SpriteScale::Quarter,
        _ => {
            warn!(
                "Sprite {:?} has a wrong height {}!",
                label.unwrap_or("???"),
                h
            );
            return Err(MapBuildError::BadSpriteHeight(h));
        }
    };

    if w == 0 || (w % h) != 0 {
        warn!(
            "Sprite {:?} has a wrong sprite ratio w={}, h={}!",
            label.unwrap_or("???"),
            w,
            h
        );

        return Err(MapBuildError::BadSpriteRatio);
    }
    Ok((scale, (w / h) as u8))
}

fn copy_texture(src_img: &Image, dest_img: &mut Image, sequence: &SpriteSeq) {
    assert!(src_img.texture_descriptor.format == TEX_FORMAT);
    assert!(src_img.texture_descriptor.format == TEX_FORMAT);

    let bytes_per_px = TEX_FORMAT.pixel_size();
    let scale_px = sequence.scale.size() as usize;
    let dst_offset_bytes_x = (sequence.x.start as usize) * scale_px * bytes_per_px;

    let src_row_bytes = src_img.size().x as usize * bytes_per_px;
    let dst_row_bytes = (TILESET_SIZE as usize) * bytes_per_px;

    for src_y in 0..scale_px {
        let dest_y = src_y + (sequence.y as usize) * scale_px;
        let src_start_bytes = src_y * src_row_bytes;
        let dest_start_bytes = dest_y * dst_row_bytes + dst_offset_bytes_x;

        let src_slice = &src_img.data[src_start_bytes..src_start_bytes + src_row_bytes];
        let dest_slice = &mut dest_img.data[dest_start_bytes..dest_start_bytes + src_row_bytes];

        dest_slice.copy_from_slice(src_slice);
    }
}
