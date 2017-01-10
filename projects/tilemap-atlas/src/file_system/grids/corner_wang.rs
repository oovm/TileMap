use super::*;
use crate::traits::check_wang4x4;
use image::{
    imageops::{resize, FilterType},
    DynamicImage, GenericImageView, RgbaImage, SubImage,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GridCornerWang {}

impl TileAtlas {
    pub(crate) fn load_grid_corner_wang(
        &self,
        root: &Path,
        target_size: u32,
        mask: u8,
        filter: FilterType,
    ) -> ImageResult<RgbaImage> {
        let image = image::open(root.join(&self.file))?.to_rgba8();
        let view = view_grid_corner_wang_cell(&image, mask);
        Ok(resize(&*view, target_size, target_size, filter))
    }
    pub(crate) fn norm_grid_corner_wang() {
        todo!()
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
