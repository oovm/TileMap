use image::error::{LimitError, LimitErrorKind};
use image::{ImageError, ImageResult, RgbaImage, SubImage};

use rand_core::RngCore;


pub trait GridAtlas {
    fn get_side(&self, l: bool, u: bool, r: bool, d: bool, n: u32) -> SubImage<&RgbaImage>;
    /// Get a tile by side relation mask.
    #[inline]
    fn get_side_random<R>(&self, l: bool, u: bool, r: bool, d: bool, rng: &mut R) -> SubImage<&RgbaImage> where R: RngCore {
        self.get_side(l, u, r, d, rng.next_u32())
    }
}

pub fn dimension_error<T>() -> ImageResult<T> {
    Err(ImageError::Limits(LimitError::from_kind(LimitErrorKind::DimensionError)))
}