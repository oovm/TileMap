use std::path::Path;
use image::{GenericImage, GenericImageView, ImageError, ImageResult, RgbaImage};
use image::error::{LimitError, LimitErrorKind};

#[cfg(feature = "serde")]
mod ser;
#[cfg(feature = "serde")]
mod der;

/// A tile set which commonly used in rpg maker
#[derive(Clone, Debug)]
pub struct TileCornerSet {
    images: [RgbaImage; 16],
}

pub struct TailCornerRandomSet {
    images: [Vec<RgbaImage>; 16],
}

impl TileCornerSet {
    pub fn new(image: &RgbaImage) -> Self {
        assert_eq!(image.width() % 4, 0, "image width {} does not divide by 4", image.width());
        assert_eq!(image.height() % 4, 0, "image height {} does not divide by 4", image.height());
        let mut out = Self {
            images: Default::default(),
        };
        // SAFETY: dimensions already checked
        for i in 0..16 {
            let x = (i % 4) as u32 * image.width() / 4;
            let y = (i / 4) as u32 * image.height() / 4;
            out.images[i] = image.view(x, y, image.width() / 4, image.height() / 4).to_image();
        }
        out
    }
    pub fn as_image(&self) -> RgbaImage {
        let (w, h) = self.cell_size();
        let mut out = RgbaImage::new(w * 4, h * 4);
        for (i, image) in self.images.iter().enumerate() {
            let x = (i % 4) as u32 * w;
            let y = (i / 4) as u32 * h;
            out.copy_from(image, x, y).unwrap();
        }
        out
    }
    pub fn save<P>(&self, path: P) -> ImageResult<()> where P: AsRef<Path> {
        self.as_image().save(path)
    }
    pub fn load<P>(path: P) -> ImageResult<Self> where P: AsRef<Path> {
        let image = image::open(path)?.to_rgba8();
        if image.width() % 4 != 0 || image.height() % 4 != 0 {
            Err(ImageError::Limits(LimitError::from_kind(LimitErrorKind::DimensionError)))?
        }
        Ok(Self::new(&image))
    }
}

impl TileCornerSet {
    pub fn cell_size(&self) -> (u32, u32) {
        let w = self.images[0].width();
        let h = self.images[0].height();
        (w, h)
    }
}

impl TileCornerSet {
    /// Create a new tile set from rpg maker atlas.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```
    /// use tilemap_atlas::TileAtlas4x6;
    /// use image::RgbaImage;
    /// ```
    pub fn from_rpg_maker(image: &RgbaImage) -> Self {
        assert_eq!(image.width() % 4, 0, "image width {} does not divide by 4", image.width());
        assert_eq!(image.height() % 6, 0, "image height {} does not divide by 6", image.height());
        let mut out = Self {
            images: Default::default(),
        };
        // SAFETY: dimensions already checked
        unsafe {
            out.make_rpg4x6(image);
        }
        out
    }
    /// Make set from RPG Maker 2000/2003 atlas
    unsafe fn make_rpg4x6(&mut self, raw: &RgbaImage) {
        self.images[00] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (0, 1), (1, 1)]);
        self.images[01] = self.make_rpg4x6_cell(&raw, [(3, 5), (1, 0), (0, 1), (1, 1)]);
        self.images[02] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 5), (0, 1), (1, 1)]);
        self.images[03] = self.make_rpg4x6_cell(&raw, [(1, 5), (2, 5), (0, 1), (1, 1)]);
        self.images[04] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (3, 2), (1, 1)]);
        self.images[05] = self.make_rpg4x6_cell(&raw, [(3, 3), (1, 0), (3, 4), (1, 1)]);
        self.images[06] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 5), (3, 2), (1, 1)]);
        self.images[07] = self.make_rpg4x6_cell(&raw, [(3, 1), (2, 5), (3, 4), (1, 1)]);
        self.images[08] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (0, 1), (0, 2)]);
        self.images[09] = self.make_rpg4x6_cell(&raw, [(3, 5), (1, 0), (0, 1), (0, 2)]);
        self.images[10] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 3), (0, 1), (0, 4)]);
        self.images[11] = self.make_rpg4x6_cell(&raw, [(1, 5), (2, 1), (0, 1), (0, 4)]);
        self.images[12] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (1, 2), (2, 2)]);
        self.images[13] = self.make_rpg4x6_cell(&raw, [(3, 3), (1, 0), (3, 0), (2, 2)]);
        self.images[14] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 3), (1, 2), (2, 0)]);
        self.images[15] = self.make_rpg4x6_cell(&raw, [(1, 3), (2, 3), (1, 4), (2, 4)]);
    }
    /// Make a cell from RPG Maker 2000/2003 atlas
    /// - index: [left up, right up, left down, right down]
    unsafe fn make_rpg4x6_cell(&self, raw: &RgbaImage, index: [(u32, u32); 4]) -> RgbaImage {
        let w = raw.width() / 4;
        let h = raw.height() / 6;
        let mut out = RgbaImage::new(w * 2, h * 2);
        for (k, (i, j)) in index.iter().enumerate() {
            for dx in 0..w {
                for dy in 0..h {
                    let x = (k as u32 % 2) * w + dx;
                    let y = (k as u32 / 2) * h + dy;
                    let pixel = raw.get_pixel(w * i + dx, h * j + dy);
                    out.put_pixel(x, y, *pixel);
                }
            }
        }
        out
    }
}

impl TileCornerSet {
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

impl TileCornerSet {}

/// Must 6 * 8 = 48
pub struct TileAtlas6x8 {
    image: RgbaImage,
}

#[test]
fn test() {
    for r in [false, true] {
        for u in [false, true] {
            for l in [false, true] {
                for d in [false, true] {
                    let idx1 = (r as u8) << 3 | (u as u8) << 2 | (l as u8) << 1 | (d as u8);
                    let lu = l && u;
                    let ru = r && u;
                    let ld = l && d;
                    let rd = r && d;
                    let idx2 = (lu as u8) << 3 | (ru as u8) << 2 | (ld as u8) << 1 | (rd as u8);
                    println!("{} -> {}", idx1, idx2)
                }
            }
        }
    }
}