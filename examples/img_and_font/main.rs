extern crate packer;
use gen::HACK_REGULAR_START;
use minimg::ImageBuffer;
use packer::*;
use std::path::Path;
use minvect::*;
use gen::*;
mod gen;

// should probos be using glow glyph roflmao

pub fn main() {
    let file_path = file!();
    let pwd = Path::new(file_path).parent().unwrap().to_str().unwrap().to_owned();
    let input_dir = pwd.clone() + "/assets/";
    let output_dir = pwd.clone() + "/packed/";
    let glyphs = r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`1234567890-=~!@#$%^&*()_+[]{}\|;:'",<.>/? "#.to_owned();
    let atlas = TextureAtlas::create(&input_dir, glyphs.clone(), 1024, 1024).unwrap();
    atlas.save(&output_dir);
    std::fs::write(pwd.clone() + "/gen.rs", atlas.generate_constants()).unwrap();

    let mut text_buf = ImageBuffer::new(1024, 1024);
    text_buf.fill(vec4(0.0, 0.0, 0.0, 1.0));
    blit_str(&mut text_buf, &atlas.buf, &glyphs, 100, 100, "The quick brown fox jumps over the lazy white-dog.");
    text_buf.dump_to_file(&(pwd.clone() + "/text_example.png"));
}

fn blit_str(buf: &mut ImageBuffer, tex: &ImageBuffer, glyphs: &str, mut x: usize, y: usize, s: &str) {
    for c in s.chars() {
        let glyph_ind = glyphs.find(c);
        if glyph_ind.is_none() { continue; }
        let glyph_ind = glyph_ind.unwrap();
        let ind = HACK_REGULAR_START + glyph_ind;
        let r_src = RECTS[ind];
        let (orig_x, orig_y) = ORIGINS[ind];
        let r_dst = PixelRect {x: (x as isize -orig_x) as usize, y: (y as isize -orig_y) as usize, w: r_src.w, h: r_src.h};
        blit(buf, tex, r_src, r_dst);
        x += r_dst.w;
    }
}

fn blit(dst: &mut ImageBuffer, src: &ImageBuffer, r_src: PixelRect, r_dst: PixelRect) {
    assert_eq!(r_src.w, r_dst.w);
    assert_eq!(r_src.h, r_dst.h);

    for i in 0..r_src.w {
        for j in 0..r_src.h {
            let x_src = r_src.x + i;
            let x_dst = r_dst.x + i;
            let y_src = r_src.y + j;
            let y_dst = r_dst.y + j;

            let src_c = src.get(x_src, y_src);
            // let src_rgb = vec3(src_c.x, src_c.y, src_c.z);
            // let dst_c = dst.get(x_dst, y_dst);
            // let dst_rgb = vec3(dst_c.x, dst_c.y, dst_c.z);

            // let blend = src_c * src_c.w + (1.0 - src_c.w) * dst_c;

            // let blended = dst_c.lerp(src_c, src)

            dst.set(x_dst, y_dst, src_c);
        }
    }
}