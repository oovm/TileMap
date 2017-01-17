#![doc=include_str!("../readme.md")]

mod traits;
pub use crate::{
    grids::{bg_set::GridBackgroundAtlas, grid_edge_set::GridEdgeOwned},
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
    grids::{corner_set::GridCornerAtlas, corner_wang::GridCornerWang, rpg_maker_xp::GridCornerRMXP},
};
