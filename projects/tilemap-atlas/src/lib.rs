#![doc=include_str!("../readme.md")]

mod traits;
pub use crate::{
    grids::{bg_set::GridBackgroundAtlas, corner_set::GridCornerAtlas, grid_edge_set::GridEdgeAtlas},
    traits::{GridAtlas, TilesProvider},
};
pub use image::{RgbaImage, SubImage};

mod file_system;
mod grids;

pub use crate::file_system::{FileSystemTiles, TileAtlasKind};
