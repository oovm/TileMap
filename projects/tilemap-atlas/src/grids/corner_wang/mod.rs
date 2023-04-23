use super::*;
use crate::GridAtlas;
mod as_complete;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridCornerWang {
    image: RgbaImage,
    cell_w: u32,
    cell_h: u32,
}

impl GridAtlas for GridCornerWang {
    unsafe fn new(image: RgbaImage) -> Self {
        let cell_w = image.width() / 4;
        let cell_h = image.height() / 4;
        Self { image, cell_w, cell_h }
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
    fn create(image: &RgbaImage, origin: (u32, u32), size: (u32, u32)) -> ImageResult<Self> {
        let (w, h) = image.dimensions();
        if w % 4 != 0 || h % 4 != 0 {
            io_error(
                "The image width must be a multiple of 4 and the image height must be a multiple of 4",
                ErrorKind::InvalidInput,
            )?;
        }
        let view = image::imageops::crop_imm(image, origin.0, origin.1, size.0 * 4, size.1 * 4);
        // SAFETY: The image has been checked.
        unsafe { Ok(Self::new(view.to_image())) }
    }

    fn get_cell_size(&self) -> (u32, u32) {
        (self.cell_w, self.cell_h)
    }

    fn get_image(&self) -> &RgbaImage {
        &self.image
    }

    fn get_by_corner(&self, lu: bool, ru: bool, ld: bool, rd: bool) -> RgbaImage {
        let (i, j) = wang4x4c_inner_mask(lu, ru, ld, rd);
        self.image.view(i * self.cell_w, j * self.cell_h, self.cell_w, self.cell_h).to_image()
    }

    fn get_by_side(&self, r: bool, u: bool, l: bool, d: bool) -> RgbaImage {
        panic!("can not get corner wang tile by side ({} {} {} {})", r, u, l, d)
    }

    fn get_by_mask(&self, mask: u8) -> RgbaImage {
        let lu = (mask >> 7) & 1 == 1;
        let ru = (mask >> 1) & 1 == 1;
        let ld = (mask >> 5) & 1 == 1;
        let rd = (mask >> 3) & 1 == 1;
        self.get_by_corner(lu, ru, ld, rd)
    }

    fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let image = image::open(path)?.to_rgba8();
        let (w, h) = image.dimensions();
        if w % 6 != 0 || h % 8 != 0 {
            io_error(
                "The image width must be a multiple of 6 and the image height must be a multiple of 8",
                ErrorKind::InvalidInput,
            )?;
        }
        Ok(Self { image, cell_w: w / 6, cell_h: h / 8 })
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
