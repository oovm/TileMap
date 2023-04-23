#![doc=include_str!("../readme.md")]
#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]

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
        bg_set::GridSimpleAtlas, complete::GridCompleteAtlas, corner_wang::GridCornerWang, edge_set::GridEdgeTiny,
        edge_wang::GridEdgeWang, rpg_maker_vx::GridCornerRMVX, rpg_maker_xp::GridCornerRMXP,
    },
    traits::{GridAtlas, TilesProvider},
};
