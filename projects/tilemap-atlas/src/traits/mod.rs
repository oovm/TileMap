use image::{
    error::{LimitError, LimitErrorKind},
    ImageError, ImageResult, RgbaImage, SubImage,
};
use std::{
    io::{Error, ErrorKind},
    path::Path,
};

use crate::{utils::save_as_png, GridCompleteAtlas};
use rand_core::RngCore;

/// A manager that can dynamically determine the required tiles.
pub trait TilesProvider {}

pub trait GridAtlas
where
    Self: Sized + Clone + Send + Sync,
    GridCompleteAtlas: From<Self>,
{
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    unsafe fn new(image: RgbaImage) -> Self;
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    fn create(image: &RgbaImage, origin: (u32, u32), size: (u32, u32)) -> ImageResult<Self>;
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    fn get_cell_size(&self) -> (u32, u32);
    /// Get the reference of raw image that owned by this tile set.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let image = raw.get_image();
    /// ```
    fn get_image(&self) -> &RgbaImage;
    ///  Create a new tile set from rpg maker xp atlas.
    ///
    /// # Arguments
    ///
    /// * `lu`: Left Up
    /// * `ru`: Right Up
    /// * `ld`: Left Down
    /// * `rd`: Right Down
    ///
    /// returns: ImageBuffer<Rgba<u8>, Vec<u8, Global>>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let image: GridCompleteAtlas = GridAtlas::load("assets/standard/grass.png").unwrap();
    /// image.save("assets/standard/grass.png").unwrap();
    /// ```
    fn get_by_corner(&self, ru: bool, rd: bool, ld: bool, lu: bool) -> RgbaImage {
        let mask = (ru as u8) << 1 | (rd as u8) << 3 | (ld as u8) << 5 | (lu as u8) << 7;
        self.get_by_mask(mask)
    }
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// # Arguments
    ///
    /// * `r`: Right
    /// * `u`: Up
    /// * `l`: Left
    /// * `d`: Down
    ///
    /// returns: ImageBuffer<Rgba<u8>, Vec<u8, Global>>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let image: GridCompleteAtlas = GridAtlas::load("assets/standard/grass.png").unwrap();
    /// image.save("assets/standard/grass.png").unwrap();
    /// ```
    fn get_by_side(&self, u: bool, r: bool, d: bool, l: bool) -> RgbaImage {
        let mask = (u as u8) << 0 | (r as u8) << 2 | (d as u8) << 4 | (l as u8) << 6;
        self.get_by_mask(mask)
    }
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// # Arguments
    ///
    /// * `mask`:
    ///
    /// returns: ImageBuffer<Rgba<u8>, Vec<u8, Global>>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let image: GridCompleteAtlas = GridAtlas::load("assets/standard/grass.png").unwrap();
    /// image.save("assets/standard/grass.png").unwrap();
    /// ```
    fn get_by_mask(&self, mask: u8) -> RgbaImage;
    /// Create the tile set from any image format, recommend use png.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let image: GridCompleteAtlas = GridAtlas::load("assets/standard/grass.png").unwrap();
    /// image.save("assets/standard/grass.png").unwrap();
    /// ```
    fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>;
    /// Save the tile set image to a png file, remember you need add `.png` suffix.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let image: GridCompleteAtlas = GridAtlas::load("assets/standard/grass.png").unwrap();
    /// image.save("assets/standard/grass.png").unwrap();
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
