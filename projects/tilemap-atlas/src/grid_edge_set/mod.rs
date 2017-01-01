use image::{GenericImageView, RgbaImage, SubImage};
use crate::GridAtlas;
use image::{GenericImage, ImageResult};
use crate::utils::dimension_error;
use std::path::Path;

mod ser;
mod der;

/// A edge tile atlas for gridded maps
///
/// It determine the pattern of the four corners of this grid according weather four sides (left, upper, left, lower) have the same elements.
///
/// ## Load
///
/// - Standard Form
/// - RPG Maker XP
/// - RPG Maker MV
///
/// ## Examples
///
/// Suppose we have such an atlas in standard form called `atlas-std.png`;
///
/// ```no_run
/// # use tileset::TailCornerAtlas;
/// let atlas = TailCornerAtlas::load("atlas-std.png").unwrap();
/// let cell = atlas.get_side(true, true, false, true);
/// cell.save("side-1011.png").unwrap();
/// ```
pub struct GridEdgeAtlas {
    image: RgbaImage,
    count: [u8; 16],
}

impl GridEdgeAtlas {
    pub fn new(image: RgbaImage, count: [u8; 16]) -> Self {
        assert_eq!(image.width() % 16, 0, "image width {} does not divide by 16", image.width());
        let cell_size = image.width() / 16;
        assert_eq!(image.height() % cell_size, 0, "image height {} does not divide by cell size {}", image.height(), cell_size);
        Self {
            image,
            count,
        }
    }
    /// Create a grid edge atlas without check
    pub unsafe fn create(image: RgbaImage, count: [u8; 16]) -> Self {
        Self {
            image,
            count,
        }
    }
}

impl GridAtlas for GridEdgeAtlas {
    fn cell_size(&self) -> u32 {
        self.image.width() / 16
    }

    fn get_cell(&self, l: bool, u: bool, r: bool, d: bool, n: u32) -> SubImage<&RgbaImage> {
        let s = self.cell_size();
        let i = ((l as u8) | (u as u8) << 1 | (r as u8) << 2 | (d as u8) << 3) as u32;
        // SAFETY: index must be in range
        let j = n % unsafe {
            *self.count.get_unchecked(i as usize) as u32
        };
        self.image.view(i * s, j * s, s, s)
    }
}