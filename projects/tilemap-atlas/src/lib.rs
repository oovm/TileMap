#![doc=include_str!("../readme.md")]
#![warn(missing_docs)]

mod traits;

pub use image::{RgbaImage, SubImage};

mod animations;
mod file_system;
mod grids;
pub mod utils;
pub use crate::{
    animations::standard::AnimationFrame,
    file_system::{FileSystemTiles, TileAtlasData},
    grids::{
        bg_set::GridSimpleAtlas, complete::GridCompleteAtlas, corner_set::GridCornerAtlas, corner_wang::GridCornerWang,
        edge_set::GridEdgeAtlas, edge_wang::GridEdgeWang, rpg_maker_vx::GridCornerRMVX, rpg_maker_xp::GridCornerRMXP,
    },
    traits::{GridAtlas, TilesProvider},
};
