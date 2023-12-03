use minimg::*;
use serde::{Serialize, Deserialize};

use crate::texture_atlas::*;
use crate::packer::*;
use crate::pixel_rect::*;
use crate::asset_reader::*;

/// Intermediate representaiton of loaded assets
pub struct AssetIR {
    pub assets: Vec<Asset>,
    pub categories: Vec<Category>,
    pub glyphs: String,
}

/// Represents a thing ready to be packed
/// for every .png file and for every chosen glyph for any identified fonts
pub struct Asset {
    pub buf: ImageBuffer,
    pub name: String,
    pub orig_x: usize,
    pub orig_y: usize,
}

/// Represents an asset category so we can generate CATEGORY_START and CATEGORY_END constants
#[derive(Clone, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub start: usize,
    pub end: usize,
}


impl AssetIR {
    pub fn try_create(assets_dir: &str, glyphs: String) -> Option<Self> {
        let mut assets = vec![];
        let mut categories = vec![];

        read_folder("", assets_dir, &glyphs, &mut assets, &mut categories);
        // read_category .
        if assets.len() != 0 {
            return Some(AssetIR{assets, categories, glyphs});
        }
        None
    }

    pub fn pack(&self, w: usize, h: usize) -> TextureAtlas {

        let mut names = vec![];
        let mut pixel_rects = vec![];

        for asset in self.assets.iter() {
            pixel_rects.push(PixelRect{x: 0, y: 0, w: asset.buf.w, h: asset.buf.h});
            names.push(asset.name.clone());            
        }
        pack_rects(&mut pixel_rects, w, h);
        let mut buf = ImageBuffer::new(w, h);
        for (idx, r) in pixel_rects.iter().enumerate() {
            for i in 0..r.w {
                for j in 0..r.h {
                    buf.set(r.x + i, r.y + j, self.assets[idx].buf.get(i, j));
                }
            }
        }

        TextureAtlas {
            buf,
            meta: TextureAtlasMetadata {
                rects: pixel_rects,
                names,
                categories: self.categories.clone(),
                glyphs: self.glyphs.clone(),
            }
        }
    }
}

// fn assert_not_intersecting(rects: &Vec<PixelRect>) {
//     for i in 0..rects.len() {
//         for j in 0..rects.len() {
//             assert!(rects[i].)
//         }
//     }
// }