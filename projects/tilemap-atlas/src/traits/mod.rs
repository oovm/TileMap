use image::{
    error::{LimitError, LimitErrorKind},
    ImageError, ImageResult, RgbaImage, SubImage,
};
use std::{
    io::{Error, ErrorKind},
    path::Path,
};

use crate::utils::save_as_png;
use rand_core::RngCore;

/// A manager that can dynamically determine the required tiles.
pub trait TilesProvider {}

pub trait GridAtlas {
    fn new(image: RgbaImage) -> ImageResult<Self>
    where
        Self: Sized;
    fn cell_size(&self) -> u32;

    fn get_image(&self) -> &RgbaImage;

    fn get_cell(&self, a: bool, b: bool, c: bool, d: bool, n: u32) -> SubImage<&RgbaImage>;
    /// Get a tile by side relation mask.
    #[inline]
    fn get_side_random<R>(&self, a: bool, b: bool, c: bool, d: bool, rng: &mut R) -> SubImage<&RgbaImage>
    where
        R: RngCore,
    {
        self.get_cell(a, b, c, d, rng.next_u32())
    }

    /// Create the tile set from any image format, recommend use png.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCompleteAtlas;
    /// let image = GridCompleteAtlas::load("assets/standard/grass.png").unwrap();
    /// image.save("assets/standard/grass.png").unwrap();
    /// ```
    fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        Self::new(image::open(path)?.to_rgba8())
    }
    /// Save the tile set image to a png file, remember you need add `.png` suffix.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCompleteAtlas;
    /// let image = GridCompleteAtlas::load("assets/grass.png").unwrap();
    /// image.save("assets/grass.png").unwrap();
    /// ```
    fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        save_as_png(self.get_image(), path)
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
