use super::*;
use image::ImageResult;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridCornerWang {
    key: String,
    cell_w: u32,
    cell_h: u32,
}

// impl TileAtlas {
//     pub(crate) fn load_grid_corner_wang(
//         &self,
//         root: &Path,
//         target_size: u32,
//         mask: u8,
//         filter: FilterType,
//     ) -> ImageResult<RgbaImage> {
//         let image = image::open(root.join(&self.file))?.to_rgba8();
//         let view = view_grid_corner_wang_cell(&image, mask);
//         Ok(resize(&*view, target_size, target_size, filter))
//     }
//     pub(crate) fn norm_grid_corner_wang() {
//         todo!()
//     }
// }

impl GridCornerWang {
    pub fn new<S>(key: S, width: u32, height: u32) -> Self
    where
        S: ToString,
    {
        Self { key: key.to_string(), cell_w: width, cell_h: height }
    }
}

impl GridCornerWang {
    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn get_path(&self, root: &Path) -> PathBuf {
        root.join(&self.key)
    }
    pub fn get_image(&self, root: &Path) -> ImageResult<RgbaImage> {
        Ok(image::open(self.get_path(root))?.to_rgba8())
    }
    pub fn get_corner(&self, root: &Path, mask: u8, target_size: u32, filter: FilterType) -> ImageResult<RgbaImage> {
        let image = self.get_image(root)?;
        let view = view_grid_corner_wang_cell(&image, mask);
        Ok(resize(&*view, target_size, target_size, filter))
    }
}

/// Get the sub image by index mask
///
/// # Arguments
///
/// * `r`: Raw image
/// * `i`: Mask of index
///
/// # Examples
///
/// ```js
/// 0b0000 <- 0  <- (1, 4)
/// 0b0001 <- 8  <- (4, 4)
/// 0b0010 <- 1  <- (1, 3)
/// 0b0011 <- 9  <- (2, 3)
/// 0b0100 <- 4  <- (1, 1)
/// 0b0101 <- 12 <- (4, 3)
/// 0b0110 <- 5  <- (3, 4)
/// 0b0111 <- 13 <- (4, 2)
/// 0b1000 <- 2  <- (2, 3)
/// 0b1001 <- 10 <- (1, 2)
/// 0b1010 <- 3  <- (2, 1)
/// 0b1011 <- 11 <- (3, 3)
/// 0b1100 <- 6  <- (4, 1)
/// 0b1101 <- 14 <- (3, 1)
/// 0b1110 <- 7  <- (2, 2)
/// 0b1111 <- 15 <- (3, 2)
/// ```
fn view_grid_corner_wang_cell(r: &RgbaImage, mask: u8) -> SubImage<&RgbaImage> {
    let s = r.width() / 4;
    match mask {
        0b0000 => r.view(0 * s, 3 * s, s, s),
        0b0001 => r.view(3 * s, 3 * s, s, s),
        0b0010 => r.view(0 * s, 2 * s, s, s),
        0b0011 => r.view(1 * s, 2 * s, s, s),
        0b0100 => r.view(0 * s, 0 * s, s, s),
        0b0101 => r.view(3 * s, 2 * s, s, s),
        0b0110 => r.view(2 * s, 3 * s, s, s),
        0b0111 => r.view(3 * s, 1 * s, s, s),
        0b1000 => r.view(1 * s, 3 * s, s, s),
        0b1001 => r.view(0 * s, 1 * s, s, s),
        0b1010 => r.view(1 * s, 0 * s, s, s),
        0b1011 => r.view(2 * s, 2 * s, s, s),
        0b1100 => r.view(3 * s, 0 * s, s, s),
        0b1101 => r.view(2 * s, 0 * s, s, s),
        0b1110 => r.view(1 * s, 1 * s, s, s),
        0b1111 => r.view(2 * s, 1 * s, s, s),
        _ => unreachable!(),
    }
}
