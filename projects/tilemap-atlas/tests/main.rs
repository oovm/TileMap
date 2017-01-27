use std::collections::BTreeMap;
use tileset::{FileSystemTiles, GridCornerAtlas, GridCornerWang, TileAtlasData};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_atlas() {
    // let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
    // debug_corner(&here.join("tests/atlas1"), |image| GridCornerOwned::from_rpg_maker_xp(image)).unwrap();
    // debug_corner(&here.join("tests/atlas2"), |image| GridCornerOwned::from_rpg_maker_xp(image)).unwrap();
    // debug_corner(&here.join("tests/atlas3"), |image| GridCornerOwned::from_wang(image)).unwrap();
}

// pub fn debug_corner<F>(root: &Path, loader: F) -> ImageResult<()>
// where
//     F: Fn(&RgbaImage) -> ImageResult<GridCornerAtlas>,
// {
//     let image = image::open(root.join("atlas.png"))?.to_rgba8();
//     let atlas = loader(&image)?;
//     atlas.save(root.join("atlas-std.png"))?;
//     for i in 0..16 {
//         let lu = (i & 1) != 0;
//         let ru = (i & 2) != 0;
//         let ld = (i & 4) != 0;
//         let rd = (i & 8) != 0;
//         let img = atlas.get_cell(lu, ru, ld, rd, 0);
//         let name = format!("corner-{}{}{}{}.png", rd as u8, ld as u8, ru as u8, lu as u8);
//         img.to_image().save(root.join(name))?;
//     }
//     Ok(())
// }

// #[test]
// fn test_edge() {
//     let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
//     debug_edge_from_wang(&here.join("tests/atlas4")).unwrap();
// }
//
// pub fn debug_edge_from_wang(root: &Path) -> ImageResult<()> {
//     let image = image::open(root.join("atlas.png"))?.to_rgba8();
//     let atlas = GridEdgeOwned::from_wang(&image)?;
//     atlas.save(root.join("atlas-std.png"))?;
//     for i in 0..16 {
//         let r = (i & 1) != 0;
//         let u = (i & 2) != 0;
//         let l = (i & 4) != 0;
//         let d = (i & 8) != 0;
//         let img = atlas.get_cell(l, u, r, d, 0);
//         let name = format!("side-{}{}{}{}.png", d as u8, r as u8, u as u8, l as u8);
//         img.to_image().save(root.join(name))?;
//     }
//     Ok(())
// }

#[test]
fn test_fs() {
    let pvd = FileSystemTiles::new("assets/tile-set-1/", 32, 32).unwrap();
    pvd.insert_atlas("atlas1", TileAtlasData::GridCornerWang(Box::new(GridCornerWang::new("a", 32, 32)))).unwrap();
    pvd.insert_atlas("atlas2", TileAtlasData::GridCornerWang(Box::new(GridCornerWang::new("b", 32, 32)))).unwrap();
    pvd.insert_atlas("atlas3", TileAtlasData::GridCornerWang(Box::new(GridCornerWang::new("c", 32, 32)))).unwrap();
}

fn debug_cell((x, y): (u32, u32), mask: &[u32]) -> String {
    let s: u32 = mask.iter().map(|i| 2u32.pow(*i)).sum();
    format!("0b{:08b} => ({}, {}),", s, x, y)
}

#[derive(Debug)]
pub struct MaskBuilder {
    map: BTreeMap<u8, (u32, u32)>,
    defaults: (u32, u32),
}

impl MaskBuilder {
    pub fn new(x: u32, y: u32) -> MaskBuilder {
        Self { map: BTreeMap::default(), defaults: (x, y) }
    }
    pub fn has_bits(&mut self, (x, y): (u32, u32), mask: &[u32]) {
        let s: u32 = mask.iter().map(|i| 2u32.pow(*i)).sum();
        let pop = self.map.insert(s as u8, (x, y));
        if let Some((i, j)) = pop {
            panic!("duplicate mask {}: new {:?}, old {:?}", s, (x, y), (i, j))
        }
    }
}

#[test]
fn test22() {
    let mut masks = MaskBuilder::new(1, 4);
    // part1
    masks.has_bits((1, 1), &[4]);
    masks.has_bits((2, 1), &[2, 4]);
    masks.has_bits((3, 1), &[2, 4, 6]);
    masks.has_bits((4, 1), &[4, 6]);
    masks.has_bits((1, 2), &[0, 4]);
    masks.has_bits((2, 2), &[0, 2, 4]);
    masks.has_bits((3, 2), &[0, 2, 4, 6]);
    masks.has_bits((4, 2), &[0, 4, 6]);
    masks.has_bits((1, 3), &[0]);
    masks.has_bits((2, 3), &[0, 2]);
    masks.has_bits((3, 3), &[0, 2, 6]);
    masks.has_bits((4, 3), &[0, 6]);
    // <default>
    masks.has_bits((2, 4), &[2]);
    masks.has_bits((3, 4), &[2, 6]);
    masks.has_bits((4, 4), &[6]);
    // part2, &[0, 1, 2, 3, 4, 5, 6, 7]
    masks.has_bits((5, 1), &[0, 2, 4, 6, 7]);
    masks.has_bits((6, 1), &[2, 3, 4, 6]);
    masks.has_bits((7, 1), &[2, 4, 5, 6]);
    masks.has_bits((8, 1), &[0, 1, 2, 4, 6]);
    masks.has_bits((5, 2), &[0, 2, 3, 4]);
    masks.has_bits((6, 2), &[0, 1, 2, 3, 4, 5, 6]);
    masks.has_bits((7, 2), &[0, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((8, 2), &[0, 4, 5, 6]);
    masks.has_bits((5, 3), &[0, 1, 2, 4]);
    masks.has_bits((6, 3), &[0, 1, 2, 3, 4, 6, 7]);
    masks.has_bits((7, 3), &[0, 1, 2, 4, 5, 6, 7]);
    masks.has_bits((8, 3), &[0, 4, 6, 7]);
    masks.has_bits((5, 4), &[0, 2, 4, 5, 6]);
    masks.has_bits((6, 4), &[0, 1, 2, 6]);
    masks.has_bits((7, 4), &[0, 2, 6, 7]);
    masks.has_bits((8, 4), &[0, 2, 3, 4, 6]);
    // part3
    masks.has_bits((9, 1), &[2, 3, 4]);
    masks.has_bits((10, 1), &[0, 2, 3, 4, 5, 6]);
    masks.has_bits((11, 1), &[2, 3, 4, 5, 6]);
    masks.has_bits((12, 1), &[4, 5, 6]);
    masks.has_bits((9, 2), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((10, 2), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((11, 2), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((12, 2), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((9, 3), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((10, 3), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((11, 3), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((12, 3), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((9, 4), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((10, 4), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((11, 4), &[0, 1, 2, 3, 4, 5, 6, 7]);
    masks.has_bits((12, 4), &[0, 1, 2, 3, 4, 5, 6, 7]);

    println!("{:#?}", masks);
}
