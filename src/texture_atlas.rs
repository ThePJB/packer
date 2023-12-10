use minimg::*;
use crate::pixel_rect::*;
use crate::packer::*;
use std::fs::*;
use std::fmt::Write;
use minvect::*;
use crate::asset_reader::*;
use crate::asset_ir::*;
use serde::{Serialize, Deserialize};

pub struct TextureAtlas {
    pub buf: ImageBuffer,
    pub meta: TextureAtlasMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct TextureAtlasMetadata {
    pub rects: Vec<PixelRect>,
    pub names: Vec<String>,
    pub categories: Vec<Category>,
    pub glyphs: String,
    pub origins: Vec<(isize, isize)>,
}

impl TextureAtlas {
    /// Save built texture atlas to file
    pub fn save(&self, path: &str) {
        let im_path = path.to_string() + "/atlas.png";
        let meta_path = path.to_string() + "/meta.json";
        let json_meta = serde_json::to_string(&self.meta).expect("failed to make meta json");
        self.buf.dump_to_file(&im_path);
        write(meta_path, &json_meta).expect("failed to write meta file");
    }

    /// Load from saved
    pub fn load(path: &str) -> Option<Self> {
        let im_path = path.to_string() + "/atlas.png";
        let meta_path = path.to_string() + "/meta.json";
        let json_meta = read_to_string(&meta_path).ok()?; //expect("failed to read rects file");
        let meta = serde_json::from_str(&json_meta)
            .expect("Failed to deserialize metadata");
        // Load the image and create the TextureAtlas
        let buf = ImageBuffer::from_bytes(&read(&im_path).expect("failed to read atlas"));
        
        Some(TextureAtlas {
            buf,
            meta,
        })
    }
    
    /// Create from assets dir
    pub fn create(asset_dir: &str, glyphs: String, w: usize, h: usize) -> Option<Self> {
        Some(AssetIR::try_create(asset_dir, glyphs)?.pack(w, h))
    }
}