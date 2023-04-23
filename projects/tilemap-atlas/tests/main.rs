use std::path::Path;
use tileset::utils::{convert_blob7x7a, convert_edge4x4, convert_rpg4x6, convert_rpg6x8, MaskBuilder};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_atlas() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
    // blob 7x7 type a
    convert_blob7x7a(&here.join("tests/blob7x7_a/bridge.png")).unwrap();
    convert_blob7x7a(&here.join("tests/blob7x7_a/commune.png")).unwrap();
    convert_blob7x7a(&here.join("tests/blob7x7_a/dungeon.png")).unwrap();
    convert_blob7x7a(&here.join("tests/blob7x7_a/islands.png")).unwrap();
    convert_blob7x7a(&here.join("tests/blob7x7_a/trench.png")).unwrap();
    convert_blob7x7a(&here.join("tests/blob7x7_a/wangbl.png")).unwrap();
    // edge 4x4
    convert_edge4x4(&here.join("tests/edge4x4/wang2e.png")).unwrap();
    convert_edge4x4(&here.join("tests/edge4x4/pipe1.png")).unwrap();
    convert_edge4x4(&here.join("tests/edge4x4/zigzag.png")).unwrap();
    convert_edge4x4(&here.join("tests/edge4x4/greek.png")).unwrap();
    convert_edge4x4(&here.join("tests/edge4x4/brickwall.png")).unwrap();
    convert_edge4x4(&here.join("tests/edge4x4/circuit.png")).unwrap();
    convert_edge4x4(&here.join("tests/edge4x4/laser.png")).unwrap();
    convert_edge4x4(&here.join("tests/edge4x4/octal.png")).unwrap();
    // corner rpg maker
    convert_rpg4x6(&here.join("tests/rpg4x6/atlas.png")).unwrap();
    convert_rpg4x6(&here.join("tests/rpg4x6/grass.png")).unwrap();
    convert_rpg6x8(&here.join("tests/rpg6x8/grass.png")).unwrap();
    convert_rpg6x8(&here.join("tests/rpg6x8/forest.png")).unwrap();
}

// #[test]
// fn test_dep() {
//     let here = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
//     decompose_image_grid_by_cells(&here.join("tests/atlas1/atlas.png"), 4, 6).unwrap();
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
//
// #[test]
// fn test_fs() {
//     let pvd = FileSystemTiles::new("assets/tile-set-1/", 32, 32).unwrap();
//     pvd.insert_atlas("atlas1", TileAtlasData::GridCornerWang(Box::new(GridCornerWang::new("a", 32, 32)))).unwrap();
//     pvd.insert_atlas("atlas2", TileAtlasData::GridCornerWang(Box::new(GridCornerWang::new("b", 32, 32)))).unwrap();
//     pvd.insert_atlas("atlas3", TileAtlasData::GridCornerWang(Box::new(GridCornerWang::new("c", 32, 32)))).unwrap();
// }

#[test]
fn test22() {
    println!("{:?}", MaskBuilder::complete_set().masks());

    println!("{}", MaskBuilder::blob7x7_set());
}
