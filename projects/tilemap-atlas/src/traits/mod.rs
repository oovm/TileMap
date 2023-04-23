use image::{ImageResult, RgbaImage};
use std::path::Path;

use crate::{utils::save_as_png, GridCompleteAtlas};

/// A manager that can dynamically determine the required tiles.
pub trait TilesProvider {}

/// Create a new tile set from rpg maker xp atlas.
pub trait GridAtlas
where
    Self: Sized + Clone + Send + Sync,
    GridCompleteAtlas: From<Self>,
{
    /// Create a new tile set without check.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// # fn main() -> image::ImageResult<()> {
    /// let image = image::open("assets/standard/grass.png")?.to_rgba8();
    /// let atlas: GridCompleteAtlas = unsafe { GridAtlas::new(image) };
    /// # Ok(())
    /// # }
    /// ```
    unsafe fn new(image: RgbaImage) -> Self;
    /// A safe way to create a new tile set.
    ///
    /// # Arguments
    ///
    /// * `image`: The raw image with alpha channel.
    /// * `origin`: The left up corner of the tile set.
    /// * `size`: The cell size of the tile set.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use image::ImageResult;
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// # fn main() -> image::ImageResult<()> {
    /// let image = image::open("assets/standard/grass.png")?.to_rgba8();
    /// let (w, h) = (image.width() / 12, image.height() / 4);
    /// let atlas: GridCompleteAtlas = GridAtlas::create(&image, (0, 0), (w, h))?;
    /// # Ok(())
    /// # }
    /// ```
    fn create(image: &RgbaImage, origin: (u32, u32), size: (u32, u32)) -> ImageResult<Self>;
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// # fn main() -> image::ImageResult<()> {
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grid-atlas.png")?;
    /// let size = raw.get_cell_size();
    /// assert_eq!(size.0, 32, "The width of the tile set is not 32");
    /// assert_eq!(size.1, 32, "The height of the tile set is not 32");
    /// # Ok(())
    /// # }
    /// ```
    fn get_cell_size(&self) -> (u32, u32);
    /// Get the reference of raw image that owned by this tile set.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// # fn main() -> image::ImageResult<()> {
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grid-atlas.png")?;
    /// let image = raw.get_image();
    /// assert_eq!(image.width(), 384, "The width of the tile set is not 384");
    /// assert_eq!(image.height(), 128, "The height of the tile set is not 128");
    /// # Ok(())
    /// # }
    /// ```
    fn get_image(&self) -> &RgbaImage;
    /// Get the tile image by corner mask.
    ///
    /// # Arguments
    ///
    /// * `ru`: 0b0001, right up corner
    /// * `rd`: 0b0010, right down corner
    /// * `ld`: 0b0100, left down corner
    /// * `lu`: 0b1000, left up corner
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// # fn main() -> image::ImageResult<()> {
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grid-atlas.png")?;
    /// let image = raw.get_by_corner(true, false, true, false);
    /// image.save("assets/grid-corner-0101.png")?;
    /// # Ok(())
    /// # }
    /// ```
    fn get_by_corner(&self, ru: bool, rd: bool, ld: bool, lu: bool) -> RgbaImage {
        let mask = (ru as u8) << 1 | (rd as u8) << 3 | (ld as u8) << 5 | (lu as u8) << 7;
        self.get_by_mask(mask)
    }
    /// Get the tile image by edge mask.
    ///
    /// # Arguments
    ///
    /// * `u`: 0b0001, up side
    /// * `r`: 0b0010, right side
    /// * `d`: 0b0100, down side
    /// * `l`: 0b1000, left side
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// # fn main() -> image::ImageResult<()> {
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grid-atlas.png")?;
    /// let image = raw.get_by_side(true, false, true, false);
    /// image.save("assets/grid-edge-0101.png")?;
    /// # Ok(())
    /// # }
    /// ```
    fn get_by_side(&self, u: bool, r: bool, d: bool, l: bool) -> RgbaImage {
        let mask = (u as u8) << 0 | (r as u8) << 2 | (d as u8) << 4 | (l as u8) << 6;
        self.get_by_mask(mask)
    }
    /// Get the tile image by grid mask.
    ///
    /// # Arguments
    ///
    /// * `0b00000001`: up side
    /// * `0b00000010`: up right corner
    /// * `0b00000100`: right side
    /// * `0b00001000`: down right corner
    /// * `0b00010000`: down side
    /// * `0b00100000`: down left corner
    /// * `0b01000000`: left side
    /// * `0b10000000`: up left corner
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// # fn main() -> image::ImageResult<()> {
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grid-atlas.png")?;
    /// let image = raw.get_by_mask(0b01010101);
    /// image.save("assets/grid-01010101.png")?;
    /// # Ok(())
    /// # }
    /// ```
    fn get_by_mask(&self, mask: u8) -> RgbaImage;
    /// Load the tile set image from disk of supported format.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// # fn main() -> image::ImageResult<()> {
    /// let bmp: GridCompleteAtlas = GridAtlas::load("assets/grid-atlas.bmp")?;
    /// bmp.save("assets/standard/grass.png")?;
    /// # Ok(())
    /// # }
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
    /// # fn main() -> image::ImageResult<()> {
    /// let bmp: GridCompleteAtlas = GridAtlas::load("assets/grid-atlas.bmp")?;
    /// bmp.save("assets/standard/grass.png")?;
    /// # Ok(())
    /// # }
    /// ```
    fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        save_as_png(self.get_image(), path)
    }
}
