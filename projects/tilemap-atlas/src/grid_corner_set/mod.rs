use image::{GenericImageView, RgbaImage, SubImage};
use crate::GridAtlas;
use crate::utils::check_width_divide_by_16;
use std::path::Path;
use image::ImageResult;

mod ser;

mod der;

mod loader;
mod display;


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
pub struct GridCornerAtlas {
    image: RgbaImage,
    count: [u8; 16],
}

impl Default for GridCornerAtlas {
    fn default() -> Self {
        Self {
            image: RgbaImage::new(16, 1),
            count: [1; 16],
        }
    }
}

impl GridCornerAtlas {
    pub fn new(image: RgbaImage, count: [u8; 16]) -> Self {
        check_width_divide_by_16(&image);
        Self {
            image,
            count,
        }
    }
    /// Create a grid corner atlas without check
    pub unsafe fn create(image: RgbaImage, count: [u8; 16]) -> Self {
        Self {
            image,
            count,
        }
    }
}

impl GridAtlas for GridCornerAtlas {
    fn cell_size(&self) -> u32 {
        self.image.width() / 16
    }

    fn get_side(&self, l: bool, u: bool, r: bool, d: bool, n: u32) -> SubImage<&RgbaImage> {
        let s = self.cell_size();
        let i = match (l, u, r, d) {
            (false, false, false, false) => { 0b0000 }
            (false, false, false, true) => { 0b0011 }
            (false, false, true, false) => { 0b1010 }
            (false, false, true, true) => { 0b1011 }
            (false, true, false, false) => { 0b0100 }
            (false, true, false, true) => { 0b0111 }
            (false, true, true, false) => { 0b1110 }
            (false, true, true, true) => { 0b1111 }
            (true, false, false, false) => { 0b1000 }
            (true, false, false, true) => { 0b1011 }
            (true, false, true, false) => { 0b1001 }
            (true, false, true, true) => { 0b1011 }
            (true, true, false, false) => { 0b1100 }
            (true, true, false, true) => { 0b1111 }
            (true, true, true, false) => { 0b1101 }
            (true, true, true, true) => { 0b1111 }
        };
        // SAFETY: index must be in range
        let j = n % unsafe {
            *self.count.get_unchecked(i as usize) as u32
        };
        self.image.view(i * s, j * s, s, s)
    }
}


#[derive(Clone, Debug)]
pub struct TailCornerAtlas {
    images: [RgbaImage; 16],
}


impl TailCornerAtlas {
    /// Get a tile by side relation mask.
    ///
    /// # Arguments
    ///
    /// - **R** = Right
    /// - **U** = Up
    /// - **L** = Left
    /// - **D** = Down
    ///
    /// returns: &ImageBuffer<Rgba<u8>, Vec<u8, Global>>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn get_side(&self, r: bool, u: bool, l: bool, d: bool) -> &RgbaImage {
        let lu = l && u;
        let ru = r && u;
        let ld = l && d;
        let rd = r && d;
        self.get_inner_corner(lu, ld, ru, rd)
    }
    /// Get a tile by corner relation mask.
    ///
    /// # Arguments
    ///
    /// - **LU** = Left Up
    /// - **LD** = Right Up
    /// - **RU** = Left Down
    /// - **RD** = Right Down
    ///
    /// returns: &ImageBuffer<Rgba<u8>, Vec<u8, Global>>
    ///
    /// # Examples
    ///
    /// ```
    /// use tilemap_atlas::TileAtlas4x6;
    /// ```
    pub fn get_inner_corner(&self, lu: bool, ru: bool, ld: bool, rd: bool) -> &RgbaImage {
        let index = (rd as u8) << 3 | (ld as u8) << 2 | (ru as u8) << 1 | (lu as u8);
        // SAFETY: index must in range `[0b0000, 0b1111]`
        unsafe {
            self.images.get_unchecked(index as usize)
        }
    }
}
