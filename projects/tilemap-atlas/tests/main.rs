use std::path::Path;
use image::{ImageResult, RgbaImage};
use tileset::{GridAtlas, TailCornerAtlas, GridEdgeAtlas, GridCornerAtlas};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_atlas() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
    debug_corner(&here.join("tests/atlas1"), |image| GridCornerAtlas::from_rpg_maker_xp(image)).unwrap();
    debug_corner(&here.join("tests/atlas2"), |image| GridCornerAtlas::from_rpg_maker_xp(image)).unwrap();
    debug_corner(&here.join("tests/atlas3"), |image| GridCornerAtlas::from_wang(image)).unwrap();
}

pub fn debug_corner<F>(root: &Path, loader: F) -> ImageResult<()> where F: Fn(&RgbaImage) -> ImageResult<GridCornerAtlas> {
    let image = image::open(root.join("atlas.png"))?.to_rgba8();
    let atlas = loader(&image)?;
    atlas.save(root.join("atlas-std.png"))?;
    for i in 0..16 {
        let lu = (i & 1) != 0;
        let ru = (i & 2) != 0;
        let ld = (i & 4) != 0;
        let rd = (i & 8) != 0;
        let img = atlas.get_cell(lu, ru, ld, rd, 0);
        let name = format!("corner-{}{}{}{}.png", rd as u8, ld as u8, ru as u8, lu as u8);
        img.to_image().save(root.join(name))?;
    }
    Ok(())
}

#[test]
fn test_edge() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
    debug_edge_from_wang(&here.join("tests/atlas4")).unwrap();
}

pub fn debug_edge_from_wang(root: &Path) -> ImageResult<()> {
    let image = image::open(root.join("atlas.png"))?.to_rgba8();
    let atlas = GridEdgeAtlas::from_wang(&image)?;
    atlas.save(root.join("atlas-std.png"))?;
    for i in 0..16 {
        let r = (i & 1) != 0;
        let u = (i & 2) != 0;
        let l = (i & 4) != 0;
        let d = (i & 8) != 0;
        let img = atlas.get_cell(l, u, r, d, 0);
        let name = format!("side-{}{}{}{}.png", d as u8, r as u8, u as u8, l as u8);
        img.to_image().save(root.join(name))?;
    }
    Ok(())
}