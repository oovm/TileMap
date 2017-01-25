use super::*;
use crate::traits::dimension_error;

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
/// # use tileset::GridCornerAtlas;
/// ```
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridCornerAtlas {
    pub(crate) key: String,
    pub(crate) cell_w: u32,
    pub(crate) cell_h: u32,
    pub(crate) count: [u8; 16],
}

/// Getters
impl GridCornerAtlas {
    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn load_image(&self, root: &Path) -> ImageResult<RgbaImage> {
        Ok(image::open(root.join(&self.key))?.to_rgba8())
    }
    pub fn load_corner(&self, root: &Path, mask: u32, index: u32) -> ImageResult<RgbaImage> {
        match self.count.get(mask as usize) {
            Some(s) if s.saturating_sub(1) >= index as u8 => {}
            _ => dimension_error()?,
        }
        let image = self.load_image(root)?;
        Ok(image.view(mask * self.cell_w, index * self.cell_h, self.cell_w, self.cell_h).to_image())
    }
}
