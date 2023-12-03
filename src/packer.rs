use minvect::*;
use serde::{Serialize, Deserialize};
use minimg::*;
use crate::pixel_rect::*;

// first sort
// then put at nearest pixel
// order must be retained from sorting
pub fn pack_rects(rects: &mut Vec<PixelRect>, w: usize, h: usize) {
    let mut keys: Vec<usize> = (0..rects.len()).collect();
    keys.sort_by(|a, b| rects[*a].cmp(&rects[*b]).reverse());
    let mut packing = Packing::new(w, h);
    for key in keys {
        rects[key] = packing.pack(rects[key]);
    }
}

pub struct Packing {
    rects: Vec<PixelRect>,
    output_w: usize,
    output_h: usize,
}

impl Packing {
    pub fn new(w: usize, h: usize) -> Self {
        Packing {
            rects: vec![],
            output_h: h,
            output_w: w,
        }

    }
    pub fn pack(&mut self, mut r: PixelRect) -> PixelRect {
        let (x, y) = self.get_space_for(r);
        r.x = x;
        r.y = y;
        self.rects.push(r);
        r
    }

    // that with minimum distance to the origin
    // i mean are we doing O(pixels)*
    // actually if we straight up scan pixels but yea skipping based on if in a rect
    // yea honestly could do that instead of my cool idea
    // i could imagine some retardation occurring here but lets just see
    pub fn get_space_for(&self, r: PixelRect) -> (usize, usize) {
        let mut i = 0;
        let mut j = 0;
        loop {
            let mut r_idx = 0;

            // loop over rects already present
            loop {
                // x gone over: drop down y
                if i + r.w >= self.output_w {
                    i = 0;
                    j += 1;
                    r_idx = 0;
                }
                // not colliding with any rects is good
                if r_idx >= self.rects.len() { 
                    break;
                }
                // colliding with a rect: skip forward
                let other_r = self.rects[r_idx];
                if other_r.intersects(&PixelRect{x: i, y: j, w: r.w, h: r.h}) {
                    i = other_r.x + other_r.w;
                    r_idx = 0;
                    continue;
                }
                r_idx += 1;    
            }
            return (i, j)
        }
    }
}





// probably need to do some packing test cases eg asserting the next space
#[test]
pub fn test_packering() {
    let mut p = Packing::new(512, 512);
    let r = PixelRect{x:0, y: 0, w: 128, h: 128};
    assert_eq!(p.get_space_for(r), (0,0));
    p.pack(r);
    assert_eq!(p.get_space_for(r), (128,0));
}