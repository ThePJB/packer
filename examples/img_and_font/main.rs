extern crate packer;
use packer::*;
use std::path::Path;

pub fn main() {
    let file_path = file!();
    let pwd = Path::new(file_path).parent().unwrap().to_str().unwrap().to_owned();
    let input_dir = pwd.clone() + "/assets/";
    let output_dir = pwd.clone() + "/packed/";
    let glyphs = r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`1234567890-=~!@#$%^&*()_+[]{}\|;:'",<.>/?"#.to_owned();
    let atlas = TextureAtlas::create(&input_dir, glyphs, 1024, 1024).unwrap();
    atlas.save(&output_dir);
    std::fs::write(pwd + "/gen.rs", atlas.generate_constants()).unwrap();
}
