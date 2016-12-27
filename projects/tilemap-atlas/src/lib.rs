mod grid_corner_set;
pub mod utils;

pub use image::{RgbaImage, SubImage};
use rand_core::RngCore;
pub use crate::grid_corner_set::{TailCornerAtlas, GridCornerAtlas};

mod grid_edge_set;
pub use crate::grid_edge_set::GridEdgeAtlas;

pub trait GridAtlas {
    fn get_side(&self, l: bool, u: bool, r: bool, d: bool, n: u32) -> SubImage<&RgbaImage>;
    fn get_side_random<R>(&self, l: bool, u: bool, r: bool, d: bool, rng: &mut R) -> SubImage<&RgbaImage> where R: RngCore {
        self.get_side(l, u, r, d, rng.next_u32())
    }
}