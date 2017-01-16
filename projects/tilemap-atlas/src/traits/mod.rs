use image::{
    error::{LimitError, LimitErrorKind},
    ImageError, ImageResult, RgbaImage, SubImage,
};
use std::io::{Error, ErrorKind};

use rand_core::RngCore;

/// A manager that can dynamically determine the required tiles.
pub trait TilesProvider {}

pub trait GridAtlas {
    fn cell_size(&self) -> u32;
    fn get_cell(&self, a: bool, b: bool, c: bool, d: bool, n: u32) -> SubImage<&RgbaImage>;
    /// Get a tile by side relation mask.
    #[inline]
    fn get_side_random<R>(&self, a: bool, b: bool, c: bool, d: bool, rng: &mut R) -> SubImage<&RgbaImage>
    where
        R: RngCore,
    {
        self.get_cell(a, b, c, d, rng.next_u32())
    }
}

pub fn dimension_error<T>() -> ImageResult<T> {
    Err(ImageError::Limits(LimitError::from_kind(LimitErrorKind::DimensionError)))
}

pub fn io_error<T, S>(message: S, kind: ErrorKind) -> ImageResult<T>
where
    S: ToString,
{
    Err(ImageError::IoError(Error::new(kind, message.to_string())))
}

pub fn check_width_divide_by_16(image: &RgbaImage) {
    assert_eq!(image.width() % 16, 0, "image width {} does not divide by 16", image.width());
    let cell_size = image.width() / 16;
    assert_eq!(image.height() % cell_size, 0, "image height {} does not divide by cell size {}", image.height(), cell_size);
}
