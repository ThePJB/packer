use minimg::*;
use ab_glyph::{FontRef, Font, Glyph};
use std::fs::{read, read_dir};
use minvect::*;

use crate::texture_atlas::*;
use crate::packer::*;
use crate::pixel_rect::*;
use crate::asset_ir::*;

// so i mean it doesnt really matter but we want basically asset metadata split from da rectz. when we save and load its the one atlas
// also idk if its correct roflmao. need to do testings
// so we might just have like vec of imbuf rects and vec of metadata and vec imbuf rects and packed imbuf rect is what is interchanged.
// different meta for once its packed too however
// and codegen is done off packed, ok cool


/// shrug emoji, should be recursive and also root is just empty string category
pub fn read_folder(prefix: &str, path: &str, glyphs: &str, assets: &mut Vec<Asset>, categories: &mut Vec<Category>) {
    let entries = read_dir(path).expect("unable to read path");
    for entry in entries {
        if entry.is_err() {
            println!("problem with entry {:?}", entry);
            continue;
        }
        let entry = entry.unwrap();
        if entry.file_type().is_err() {
            println!("problem with entry file type {:?}", entry);
            continue;
        }
        let file_type = entry.file_type().unwrap();
        let name = entry.file_name().to_str().unwrap().to_owned();
        if file_type.is_dir() {
            let new_prefix = if prefix == "" {
                name.clone()
            } else {
                prefix.to_owned() + "_" + &name
            };
            let category_name = name.clone();
            let start = assets.len();
            read_folder(&new_prefix, &(path.to_owned() + "/" + &name), glyphs, assets, categories);
            let end = assets.len() - 1;
            let category = Category {
                name: category_name,
                start,
                end,
            };
            categories.push(category);
        } else {
            let file_path = path.to_owned() + "/" + &name;
            if let Some((just_name, extension)) = name.split_once('.') {
                let name = if prefix == "" {
                    just_name.to_owned()
                } else {
                    prefix.to_owned() + "_" + just_name
                };
                if extension == "png" {
                    let buf = ImageBuffer::from_bytes(&read(&file_path).expect("failed to read"));    // where is my ? that just does continue roflmao
                    let w = buf.w;
                    let h = buf.h;
                    let asset = Asset {
                        buf,
                        name,
                        orig_x: w / 2,
                        orig_y: h,
                    };
                    assets.push(asset);
                } else if extension == "ttf" {
                    let font_bytes = read(&file_path).expect("failed to read");
                    let font = FontRef::try_from_slice(&font_bytes).expect("failed to font");
                    let category_name = name.clone();
                    let start = assets.len();
                    for c in glyphs.chars() {
                        let name = name.clone() + "_" + &c.to_string();
                        let glyph: Glyph = font.glyph_id(c).with_scale(48.0);   // roflmao scale idk
                        let og = font.outline_glyph(glyph);
                        if og.is_none() {
                            panic!("fuck");
                        }
                        let og = og.unwrap();
                        let r = og.px_bounds();
                        let w = r.width() as usize;
                        let h = r.height() as usize;
                        let mut buf = ImageBuffer::new(w, h);
                        let orig_x = -r.min.x as usize;
                        let orig_y = -r.min.y as usize;
                        // yea so defos pretty confused about this shit e.g. its u32 so does that mean negatives get clipped smh. lets hope it good
                        og.draw(|x, y, c| {
                            let colour = vec4(1.0, 1.0, 1.0, c); // will it blend lmao
                            buf.set(x as usize, y as usize, colour)
                        });
                        let glyph_asset = Asset {
                            buf,
                            orig_x,
                            orig_y,
                            name: "__".to_owned() + &name,
                        };
                        assets.push(glyph_asset);
                    }
                    let end = assets.len() - 1;
                    let category = Category {
                        name: category_name,
                        start,
                        end,
                    };
                    categories.push(category);
                }
            }
            // split on . n get suffix
            // if png push that shit
            // if ttf, push glyphs * ttf roflmao and category things
            // maybe wont do meta file atm but it bears consideration
        }
        // order shit: eg folders first, alphabetical, then items, alphabetical?
    }
}