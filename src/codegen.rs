use crate::texture_atlas::*;
use crate::asset_ir::*;
use minvect::*;

impl TextureAtlas {
    // this is like v1, v2 i want to have categories
    // each category has FIRST_CATEGORYNAME and LAST_CATEGORYNAME constants in addition
    // otherwise prepending stuff etc
    // honestly like procedural version of this code is probably cleaner and faster
    
    pub fn generate_constants(&self) -> String {
        let consts = self.meta.names.iter()
            .map(|s| s.to_uppercase())
            .enumerate()
            .filter(|(i, s)| !s.starts_with("__"))
            .map(|(i, s)| format!("pub const {}: usize = {};", s, i))
            .fold(String::new(), |a, b| a + &b + "\n");
        
        let clips = "pub const CLIPS: &[Rect] = &[\n".to_owned() + &self.meta.rects.iter()
            .map(|r| rect(r.x as f32 / self.buf.w as f32, r.y as f32 /self.buf.h as f32, r.w as f32 / self.buf.w as f32, r.h as f32 / self.buf.h as f32))
            .map(|r| rect_generating_string(&r))
            .fold(String::new(), |a, b| a + &b) + "];\n";
        
        let rects = "pub const RECTS: &[PixelRect] = &[\n".to_owned() + &self.meta.rects.iter()
            .map(|r| "\tPixelRect{x:".to_owned() + &r.x.to_string() + ", y:" + &r.y.to_string() + ", w:" + &r.w.to_string() + ", h:" + &r.h.to_string() + "},\n")
            .fold(String::new(), |a, b| a + &b) + "];\n";
        
        // let names = "pub const NAMES: &[&str] = &[\n".to_owned() + &self.meta.names.iter().filter(|s| !s.starts_with("__")).map(|s| "\t\"".to_owned() + &s.to_uppercase() + "\",\n")
        //     .fold(String::new(), |a, b| a + &b) + "];\n";
        
        let categories = self.meta.categories.iter()
            .map(|Category{name, start, end}| {
                let name = name.to_uppercase();
                "pub const ".to_owned() + &name + "_START: usize = " + &start.to_string() + ";\n" +     
                "pub const " + &name + "_END: usize = " + &end.to_string() + ";\n"     
            })
            .fold(String::new(), |a, b| a + &b);

        "use minvect::*;\n".to_owned() + &consts + "\n" + &categories + "\n" + &clips + "\n" + &rects
    }
}

fn rect_generating_string(r: &Rect) -> String {
    let mut buf = "\trect(".to_owned();
    for x in [r.xy.x, r.xy.y, r.wh.x, r.wh.y] {
        let s = x.to_string();
        let s = if s.contains('.') {
            s
        } else {
            s + "."
        };
        buf += &(s + ",");
    }
    buf += "),\n";
    buf
}