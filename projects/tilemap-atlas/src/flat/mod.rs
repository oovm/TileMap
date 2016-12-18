use std::cell::OnceCell;
use std::path::Path;
use image::{GenericImage, ImageError, ImageResult, Rgba, RgbaImage};

mod ser;
mod der;

/// Must 6 * 8 = 48
#[derive(Clone, Debug)]
pub struct TileAtlas4x6 {
    image: RgbaImage,
    cache: [RgbaImage; 16],
}

impl TileAtlas4x6 {
    /// Create a new tile atlas from a image.
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
    pub fn new(image: RgbaImage) -> Self {
        assert_eq!(image.width() % 4, 0, "image width {} does not divide by 4", image.width());
        assert_eq!(image.height() % 6, 0, "image height {} does not divide by 6", image.height());
        let mut out = Self {
            image,
            cache: Default::default(),
        };
        out.make_cache();
        out
    }
    pub fn load<P>(path: P) -> ImageResult<Self> where P: AsRef<Path> {
        let image = image::open(path)?.to_rgba8();

        Ok(Self::new(image))
    }
    pub fn save<P>(&self, path: P) -> ImageResult<()> where P: AsRef<Path> {
        self.image.save(path)
    }
}

impl TileAtlas4x6 {
    pub fn get_pattern(&self, lu: bool, ld: bool, ru: bool, rd: bool) -> &RgbaImage {
        let index = (lu as u8) << 3 | (ld as u8) << 2 | (ru as u8) << 1 | (rd as u8);
        // SAFETY: index must in range `[0b0000, 0b1111]`
        unsafe {
            self.cache.get_unchecked(index as usize)
        }
    }
    pub fn get(&self, index: usize) -> &RgbaImage {
        match self.cache.get(index) {
            Some(s) => {
                s
            }
            None => {
                panic!("index must in range `[0b0000, 0b1111]`");
            }
        }
    }
    fn cell_width(&self) -> u32 {
        self.image.width() / 4
    }
    fn cell_height(&self) -> u32 {
        self.image.height() / 6
    }
    fn make_cache(&mut self) {
        self.cache[00] = self.make_cell([(0, 0), (1, 0), (0, 1), (1, 1)]);
        self.cache[01] = self.make_cell([(3, 5), (1, 0), (0, 1), (1, 1)]);
        self.cache[02] = self.make_cell([(0, 0), (0, 5), (0, 1), (1, 1)]);
        self.cache[03] = self.make_cell([(1, 5), (2, 5), (0, 1), (1, 1)]);
        self.cache[04] = self.make_cell([(0, 0), (1, 0), (3, 2), (1, 1)]);
        self.cache[05] = self.make_cell([(3, 3), (1, 0), (3, 4), (1, 1)]);
        self.cache[06] = self.make_cell([(0, 0), (0, 5), (3, 2), (1, 1)]);
        self.cache[07] = self.make_cell([(3, 1), (2, 5), (3, 4), (1, 1)]);
        self.cache[08] = self.make_cell([(0, 0), (1, 0), (0, 1), (0, 2)]);
        self.cache[09] = self.make_cell([(3, 5), (1, 0), (0, 1), (0, 2)]);
        self.cache[10] = self.make_cell([(0, 0), (0, 3), (0, 1), (0, 4)]);
        self.cache[11] = self.make_cell([(1, 5), (2, 1), (0, 1), (0, 4)]);
        self.cache[12] = self.make_cell([(0, 0), (1, 0), (1, 2), (2, 2)]);
        self.cache[13] = self.make_cell([(3, 3), (1, 0), (3, 0), (2, 2)]);
        self.cache[14] = self.make_cell([(0, 0), (0, 3), (1, 2), (2, 0)]);
        self.cache[15] = self.make_cell([(1, 3), (2, 3), (1, 4), (2, 4)]);
    }
    // [left up, right up, left down, right down]
    fn make_cell(&self, index: [(u32, u32); 4]) -> RgbaImage {
        let mut image = RgbaImage::new(self.cell_width() * 2, self.cell_height() * 2);
        for (i, j) in index.iter() {
            for dx in 0..self.cell_width() {
                for dy in 0..self.cell_height() {
                    // unsafe {
                    //     let pixel = self.image.get_unchecked((sx + x, sy + y));
                    //     image.get_unchecked_mut((x, y)).clone_from(pixel);
                    // }
                    // safe version
                    let x = i * self.cell_width() + dx;
                    let y = j * self.cell_height() + dy;
                    let pixel = self.image.get_pixel(x, y);
                    image.put_pixel(dx, dy, *pixel);
                }
            }
        }
        image
    }
}


/// Must 6 * 8 = 48
pub struct TileAtlas6x8 {
    image: RgbaImage,
}