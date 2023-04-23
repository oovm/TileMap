use super::*;
use crate::GridAtlas;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridCornerWang {
    image: RgbaImage,
    cell_w: u32,
    cell_h: u32,
}

impl GridAtlas for GridCornerWang {
    fn new(image: RgbaImage) -> ImageResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    fn cell_size(&self) -> u32 {
        todo!()
    }

    fn get_image(&self) -> &RgbaImage {
        todo!()
    }

    fn get_cell(&self, a: bool, b: bool, c: bool, d: bool, n: u32) -> SubImage<&RgbaImage> {
        todo!()
    }
}

// constructors
impl GridCornerWang {
    /// Create a complete tile set from image.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCompleteAtlas;
    /// let image = image::open("assets/standard/grass.png").unwrap().to_rgba8();
    /// let tile_set = GridCompleteAtlas::new(image).unwrap();
    /// ```
    pub fn new(image: RgbaImage) -> ImageResult<Self> {
        let (w, h) = image.dimensions();
        if w % 12 != 0 || h % 4 != 0 {
            io_error(
                "The image width must be a multiple of 12 and the image height must be a multiple of 4",
                ErrorKind::InvalidInput,
            )?;
        }
        // SAFETY: The image has been checked.
        unsafe { Ok(Self::create(image)) }
    }
    /// Create a complete tile set without check.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCompleteAtlas;
    /// let image = image::open("assets/standard/grass.png").unwrap().to_rgba8();
    /// let tile_set = unsafe { GridCompleteAtlas::create(image) };
    /// ```
    pub unsafe fn create(image: RgbaImage) -> Self {
        let cell_w = image.width() / 12;
        let cell_h = image.height() / 4;
        Self { image, cell_w, cell_h }
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
    pub fn load<P>(path: P) -> ImageResult<Self>
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
    pub fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        save_as_png(&self.image, path)
    }
}

// getters
impl GridCornerWang {
    /// Get Image
    ///
    /// # Arguments
    ///
    /// * `root`:
    ///
    /// returns: Result<ImageBuffer<Rgba<u8>, Vec<u8, Global>>, ImageError>
    ///
    /// # Examples
    ///
    /// ```
    /// # use tileset::GridCornerWang;
    /// ```
    pub fn get_by_corner(&self, lu: bool, ru: bool, ld: bool, rd: bool) -> RgbaImage {
        let (i, j) = wang4x4c_inner_mask(lu, ru, ld, rd);
        self.image.view(i * self.cell_w, j * self.cell_h, self.cell_w, self.cell_h).to_image()
    }
    pub fn get_by_mask(&self, mask: u8) -> RgbaImage {
        let lu = (mask >> 7) & 1 == 1;
        let ru = (mask >> 1) & 1 == 1;
        let ld = (mask >> 5) & 1 == 1;
        let rd = (mask >> 3) & 1 == 1;
        self.get_by_corner(lu, ru, ld, rd)
    }
}

pub fn get_by_mask(mask: u8) -> (u32, u32) {
    let lu = (mask >> 7) & 1 == 1;
    let ru = (mask >> 1) & 1 == 1;
    let ld = (mask >> 5) & 1 == 1;
    let rd = (mask >> 3) & 1 == 1;
    wang4x4c_inner_mask(lu, ru, ld, rd)
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
pub fn wang4x4c_inner_mask(lu: bool, ru: bool, ld: bool, rd: bool) -> (u32, u32) {
    /// match [bool;4] directly has too many branch jumps
    let mask = (lu as u8) << 0 | (ru as u8) << 1 | (ld as u8) << 2 | (rd as u8) << 3;
    match mask {
        0b0000 => (0, 3),
        0b0001 => (3, 3),
        0b0010 => (0, 2),
        0b0011 => (1, 2),
        0b0100 => (0, 0),
        0b0101 => (3, 2),
        0b0110 => (2, 3),
        0b0111 => (3, 1),
        0b1000 => (1, 3),
        0b1001 => (0, 1),
        0b1010 => (1, 0),
        0b1011 => (2, 2),
        0b1100 => (3, 0),
        0b1101 => (2, 0),
        0b1110 => (1, 1),
        0b1111 => (2, 1),
        _ => unreachable!(),
    }
}
