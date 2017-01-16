use super::*;
use serde::{Deserializer, Serializer};

/// A tile atlas for gridded maps
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
#[derive(Clone, Debug)]
pub struct GridCornerOwned {
    image: RgbaImage,
    count: [u8; 16],
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridCornerAtlas {
    pub(crate) key: String,
    pub(crate) cell_w: u32,
    pub(crate) cell_h: u32,
    pub(crate) count: [u32; 16],
}

impl Default for GridCornerOwned {
    fn default() -> Self {
        Self { image: RgbaImage::new(16, 1), count: [1; 16] }
    }
}

impl GridCornerOwned {
    pub fn new(image: RgbaImage, count: [u8; 16]) -> Self {
        check_width_divide_by_16(&image);
        Self { image, count }
    }
    /// Create a grid corner atlas without check
    pub unsafe fn create(image: RgbaImage, count: [u8; 16]) -> Self {
        Self { image, count }
    }
}

impl GridAtlas for GridCornerOwned {
    fn cell_size(&self) -> u32 {
        self.image.width() / 16
    }

    fn get_cell(&self, lu: bool, ru: bool, ld: bool, rd: bool, n: u32) -> SubImage<&RgbaImage> {
        let s = self.cell_size();
        let i = ((lu as u8) | (ru as u8) << 1 | (ld as u8) << 2 | (rd as u8) << 3) as u32;
        // SAFETY: index must be in range
        let j = n % unsafe { *self.count.get_unchecked(i as usize) as u32 };
        self.image.view(i * s, j * s, s, s)
    }
}
