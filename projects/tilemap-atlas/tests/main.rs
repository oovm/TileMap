use std::path::Path;
use tilemap_atlas::TileAtlas4x6;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn image() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
    debug_atlas4x6(&here.join("tests/atlas1-4x6"));
}

pub fn debug_atlas4x6(root: &Path) {
    let atlas = TileAtlas4x6::load(root.join("atlas.png")).unwrap();
    for i in 0..16 {
        let r = (i & 8) != 0;
        let u = (i & 4) != 0;
        let l = (i & 2) != 0;
        let d = (i & 1) != 0;
        let img = atlas.get_side(r, u, l, d);
        let name = format!("side-{}{}{}{}.png", r as u8, u as u8, l as u8, d as u8);
        img.save(root.join(name)).unwrap();
    }
    for i in 0..16 {
        let lu = (i & 8) != 0;
        let ru = (i & 4) != 0;
        let ld = (i & 2) != 0;
        let rd = (i & 1) != 0;
        let img = atlas.get_corner(lu, ru, ld, rd);
        let name = format!("corner-{}{}{}{}.png", lu as u8, ru as u8, ld as u8, rd as u8);
        img.save(root.join(name)).unwrap();
    }
}