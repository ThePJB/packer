use serde::{Serialize, Deserialize};

#[derive(PartialOrd, Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PixelRect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

use std::{cmp::Ordering};
impl Ord for PixelRect {
    fn cmp(&self, other: &Self) -> Ordering {
        let area_cmp = (self.w*self.h).cmp(&(other.w*other.h));
        if area_cmp == Ordering::Equal {
            return self.w.cmp(&other.w);
        }
        return area_cmp;
    }
}

impl PixelRect {
    pub fn contains(&self, x: usize, y: usize) -> bool {
        self.x <= x && self.x + self.w > x &&
        self.y <= y && self.y + self.h > y
    }
    pub fn intersects(&self, other: &PixelRect) -> bool {
        !(self.x + self.w <= other.x || self.x >= other.x + other.w || self.y + self.h <= other.y || self.y >= other.y + other.h)
    }
}