#![doc=include_str!("../readme.md")]

mod traits;
pub use crate::{
    grids::{bg_set::GridBackgroundAtlas, corner_set::GridCornerAtlas, grid_edge_set::GridEdgeAtlas},
    traits::{GridAtlas, TilesProvider},
};
pub use image::{RgbaImage, SubImage};

mod animations;
mod file_system;
mod grids;
pub mod utils;

pub use crate::{
    animations::standard::AnimationFrame,
    file_system::{FileSystemTiles, TileAtlasData, TileAtlasKind},
    grids::corner_wang::GridCornerWang,
};
