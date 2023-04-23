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
    /// # use tileset::GridCompleteAtlas;
    /// let image = image::open("assets/standard/grass.png").unwrap().to_rgba8();
    /// let tile_set = unsafe { GridCompleteAtlas::create(image) };
    /// ```
    fn create(image: &RgbaImage, origin: (u32, u32), size: (u32, u32)) -> ImageResult<Self> {
        let (w, h) = image.dimensions();
        if origin.0 + size.0 * 4 > w || origin.1 + size.1 * 4 > h {
            io_error("The image size has out of range", ErrorKind::InvalidInput)?;
        }
        let view = image::imageops::crop_imm(image, origin.0, origin.1, size.0 * 4, size.1 * 4);
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
        todo!()
    }

    fn get_by_mask(&self, mask: u8) -> RgbaImage {
        todo!()
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

// 0b0000 <- 0  <- (1, 4)
// 0b0001 <- 2  <- (2, 4)
// 0b0010 <- 1  <- (1, 3)
// 0b0011 <- 3  <- (2, 3)
// 0b0100 <- 8  <- (4, 4)
// 0b0101 <- 10 <- (3, 4)
// 0b0110 <- 9  <- (4, 3)
// 0b0111 <- 11 <- (3, 3)
// 0b1000 <- 4  <- (1, 1)
// 0b1001 <- 6  <- (2, 1)
// 0b1010 <- 5  <- (1, 2)
// 0b1011 <- 7  <- (2, 2)
// 0b1100 <- 12 <- (4, 1)
// 0b1101 <- 14 <- (3, 1)
// 0b1110 <- 13 <- (4, 2)
// 0b1111 <- 15 <- (3, 2)
fn view_wang4x4e_cell(r: &RgbaImage, mask: u8) -> SubImage<&RgbaImage> {
    let w = r.width() / 4;
    let h = r.height() / 4;
    match mask {
        0b0000 => r.view(0 * w, 3 * h, w, h),
        0b0001 => r.view(1 * w, 3 * h, w, h),
        0b0010 => r.view(0 * w, 2 * h, w, h),
        0b0011 => r.view(1 * w, 2 * h, w, h),
        0b0100 => r.view(3 * w, 3 * h, w, h),
        0b0101 => r.view(2 * w, 3 * h, w, h),
        0b0110 => r.view(3 * w, 2 * h, w, h),
        0b0111 => r.view(2 * w, 2 * h, w, h),
        0b1000 => r.view(0 * w, 0 * h, w, h),
        0b1001 => r.view(1 * w, 0 * h, w, h),
        0b1010 => r.view(0 * w, 1 * h, w, h),
        0b1011 => r.view(1 * w, 1 * h, w, h),
        0b1100 => r.view(3 * w, 0 * h, w, h),
        0b1101 => r.view(2 * w, 0 * h, w, h),
        0b1110 => r.view(3 * w, 1 * h, w, h),
        0b1111 => r.view(2 * w, 1 * h, w, h),
        _ => unreachable!(),
    }
}
