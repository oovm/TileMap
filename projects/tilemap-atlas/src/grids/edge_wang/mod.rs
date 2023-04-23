use super::*;

mod as_complete;

/// Create a complete tile set without check.
///
/// # Examples
///
/// ```no_run
/// # use tileset::GridCompleteAtlas;
/// let image = image::open("assets/standard/grass.png").unwrap().to_rgba8();
/// let tile_set = unsafe { GridCompleteAtlas::create(image) };
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridEdgeWang {
    image: RgbaImage,
}

impl GridAtlas for GridEdgeWang {
    unsafe fn new(image: RgbaImage) -> Self {
        Self { image }
    }
    /// Create a complete tile set without check.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCornerWang};
    /// let image = image::open("assets/standard/grass.png").unwrap().to_rgba8();
    /// let tile_set =
    ///     GridCornerWang::create(&image, (0, 0), (image.width() / 4, image.height() / 4)).unwrap();
    /// ```
    fn create(image: &RgbaImage, (x, y): (u32, u32), (w, h): (u32, u32)) -> ImageResult<Self> {
        let (image_w, image_h) = image.dimensions();
        if x + w * 4 > image_w || y + h * 4 > image_h {
            io_error("The image size has out of range", ErrorKind::InvalidInput)?;
        }
        let view = image.view(x, y, w * 4, h * 4);
        // SAFETY: The image has been checked.
        unsafe { Ok(Self::new(view.to_image())) }
    }

    fn get_cell_size(&self) -> (u32, u32) {
        (self.image.width() / 4, self.image.height() / 4)
    }

    fn get_image(&self) -> &RgbaImage {
        &self.image
    }

    fn get_by_corner(&self, ru: bool, rd: bool, ld: bool, lu: bool) -> RgbaImage {
        panic!("can not get edge wang tile by corner ({} {} {} {})", ru, rd, ld, lu)
    }

    fn get_by_side(&self, u: bool, r: bool, d: bool, l: bool) -> RgbaImage {
        let (i, j) = wang4x4e_sub_image(u, r, d, l);
        let (w, h) = self.get_cell_size();
        let view = self.image.view(i * w, j * h, w, h);
        view.to_image()
    }

    fn get_by_mask(&self, mask: u8) -> RgbaImage {
        let u = mask >> 0 & 1 == 1;
        let r = mask >> 2 & 1 == 1;
        let d = mask >> 4 & 1 == 1;
        let l = mask >> 6 & 1 == 1;
        self.get_by_side(u, r, d, l)
    }

    fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let image = image::open(path)?.to_rgba8();
        let (w, h) = image.dimensions();
        if w % 4 != 0 || h % 4 != 0 {
            io_error(
                "The image width must be a multiple of 6 and the image height must be a multiple of 8",
                ErrorKind::InvalidInput,
            )?;
        }
        Ok(Self { image })
    }
}
/// ```js
/// 0b0000 <- 0  <- (1, 4)
/// 0b0001 <- 1  <- (1, 3)
/// 0b0010 <- 2  <- (2, 4)
/// 0b0011 <- 3  <- (2, 3)
/// 0b0100 <- 4  <- (1, 1)
/// 0b0101 <- 5  <- (1, 2)
/// 0b0110 <- 6  <- (2, 1)
/// 0b0111 <- 7  <- (2, 2)
/// 0b1000 <- 8  <- (4, 4)
/// 0b1001 <- 9  <- (4, 3)
/// 0b1010 <- 10 <- (3, 4)
/// 0b1011 <- 11 <- (3, 3)
/// 0b1100 <- 12 <- (4, 1)
/// 0b1101 <- 13 <- (4, 2)
/// 0b1110 <- 14 <- (3, 1)
/// 0b1111 <- 15 <- (3, 2)
/// ```
fn wang4x4e_sub_image(u: bool, r: bool, d: bool, l: bool) -> (u32, u32) {
    let mask = (u as u8) << 0 | (r as u8) << 1 | (d as u8) << 2 | (l as u8) << 3;
    match mask {
        0b0000 => (0, 3),
        0b0001 => (0, 2),
        0b0010 => (1, 3),
        0b0011 => (1, 2),
        0b0100 => (0, 0),
        0b0101 => (0, 1),
        0b0110 => (1, 0),
        0b0111 => (1, 1),
        0b1000 => (3, 3),
        0b1001 => (3, 2),
        0b1010 => (2, 3),
        0b1011 => (2, 2),
        0b1100 => (3, 0),
        0b1101 => (3, 1),
        0b1110 => (2, 0),
        0b1111 => (2, 1),
        _ => unreachable!(),
    }
}
