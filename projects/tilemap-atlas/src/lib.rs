#![doc=include_str!("../readme.md")]

mod utils;
pub use crate::{
    grids::{bg_set::GridBackgroundAtlas, corner_set::GridCornerAtlas, grid_edge_set::GridEdgeAtlas},
    utils::{GridAtlas, TilesProvider},
};
pub use image::{RgbaImage, SubImage};

mod grids;
mod sets;

pub use sets::{AtlasReference, FileSystemTiles};
