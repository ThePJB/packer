ImageBuffers from FS: 
- yes read them
- put in a slice

ImageBuffers from TTF glyphs: path to ttf & string of what glyphs u want 

r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`1234567890-=~!@#$%^&*()_+[]{}\|;:'",<.>/?"#
OK but how would we reference them tho? maybe just with index shit eg call em GLYPH_1, GLYPH_2 etc but yea get em relative to da string


in any case inner pack logic needs to take a slice of [ImageBuffer, (any of name category? not wh imbuf has wh) (offset, could come in .offset file too)]
glyphs will have offset

glyphs can just be anon and its GLYPH_START, GLYPH_END
n yea idk if wanting hashmap or linear probe shrug
and multiple fonts? maybe HACK_START and HACK_END for example
oi yea support for .ttfs found in dir essentially

# Usage
Organize assets folder with .png or .ttf assets, at root level or in 1 level deep 'category' folders
this will generated a big texture atlas contianing all, plus generate a constants file to statically reference clips, width/height, name, etc
(And this is where category comes in) constants file will also include start/end of each category
stuff will be ordered alphabetically so if u need do 000_dirt.png etc

sub categories ay ahh yer y not. stat end for each category or sub category

## TODO
nb this will be fucked for shit like if missing characters so just be careful
maybe unicode_names2 or just anonymize font names
mkdir -p output_dir
codegen n

## Example
```
let input_dir = "./assets/";
let output_dir = "./packed/";
let glyphs = r#"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`1234567890-=~!@#$%^&*()_+[]{}\|;:'",<.>/?"#.to_owned();
let atlas = TextureAtlas::create(input_dir, glyphs, 1024, 1024).unwrap();
atlas.save(output_dir);
```