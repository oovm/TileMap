use super::*;

mod as_complete;

/// Wang tile connected by corner rule.
///
/// ![](https://raw.githubusercontent.com/oovm/TileMap/dev/projects/tilemap-atlas/assets/documents/1x/corner-case.png)
///
/// ![](https://raw.githubusercontent.com/oovm/TileMap/dev/projects/tilemap-atlas/assets/documents/1x/complete-atlas.png)
///
/// # Examples
///
///
/// ```no_run
/// # use tileset::{GridAtlas, GridCornerWang};
/// let image = image::open("assets/standard/grass.png").unwrap().to_rgba8();
/// let tile_set =
///     GridCornerWang::create(&image, (0, 0), (image.width() / 4, image.height() / 4)).unwrap();
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridCornerWang {
    image: RgbaImage,
}

impl GridAtlas for GridCornerWang {
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
        let (w, h) = self.get_cell_size();
        let (i, j) = wang4x4c_inner_mask(lu, ru, ld, rd);
        self.image.view(i * w, j * h, w, h).to_image()
    }

    fn get_by_side(&self, u: bool, r: bool, d: bool, l: bool) -> RgbaImage {
        panic!("can not get corner wang tile by side ({} {} {} {})", r, u, l, d)
    }
    fn get_by_mask(&self, mask: u8) -> RgbaImage {
        let lu = mask >> 7 & 1 == 1;
        let ru = mask >> 1 & 1 == 1;
        let ld = mask >> 5 & 1 == 1;
        let rd = mask >> 3 & 1 == 1;
        self.get_by_corner(lu, ru, ld, rd)
    }

    fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let image = image::open(path)?.to_rgba8();
        check_image_multiple(&image, 4, 4)?;
        Ok(Self { image })
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
    // match [bool;4] directly has too many branch jumps
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
