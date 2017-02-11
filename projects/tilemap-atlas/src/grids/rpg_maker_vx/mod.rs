use super::*;
mod to_complete;

/// A corner type tile set used in [RPG Maker VX](), [RPG MakerMV](), [RPG MakerMZ]().
///
/// ## Example
///
/// ![]()
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridCornerRMVX {
    image: RgbaImage,
    cell_w: u32,
    cell_h: u32,
}

impl GridCornerRMVX {
    /// Create a new [`GridCornerRMVX`] tile set from rpg maker atlas.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```
    /// use tileset::GridCornerRMVXFile;
    /// ```
    pub fn new(image: &RgbaImage, (x, y): (u32, u32), (w, h): (u32, u32)) -> ImageResult<Self> {
        let max_x = x + 4 * w;
        let max_y = y + 6 * h;
        if max_x > image.width() || max_y > image.height() {
            io_error("The image size has out of range", ErrorKind::InvalidInput)?;
        }
        // SAFETY: The image has been checked.
        unsafe { Ok(Self::create(image, (x, y), (w, h))) }
    }
    /// Create a new [`GridCornerRMVX`] tile set without check.
    ///
    /// # Arguments
    ///
    /// * `image`:
    /// * `(x, y)`:
    /// * `(w, h)`:
    ///
    /// returns: GridCornerRMMV
    ///
    /// # Examples
    ///
    /// ```
    /// use tileset::GridCornerRMVXFile;
    /// ```
    pub unsafe fn create(image: &RgbaImage, (x, y): (u32, u32), (w, h): (u32, u32)) -> Self {
        let image = image::imageops::crop_imm(image, x, y, w * 4, h * 6).to_image();
        Self { image, cell_w: w, cell_h: h }
    }
    pub fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let image = image::open(path)?.to_rgba8();
        let (w, h) = image.dimensions();
        if w % 4 != 0 || h % 6 != 0 {
            io_error(
                "The image width must be a multiple of 4 and the image height must be a multiple of 6",
                ErrorKind::InvalidInput,
            )?;
        }
        Ok(Self { image, cell_w: w / 4, cell_h: h / 6 })
    }
}

// pub fn from_rpg_maker_xp(rpg: &RgbaImage) -> ImageResult<Self> {
//     assert_eq!(rpg.width() % 4, 0, "image width {} does not divide by 4", rpg.width());
//     assert_eq!(rpg.height() % 6, 0, "image height {} does not divide by 6", rpg.height());
//     let half_cell = rpg.width() / 4;
//     let mut out = Self { image: RgbaImage::new(half_cell * 16 * 2, half_cell * 2), count: [1; 16] };
//     for i in 0..16 {
//         let view = view_rpg4x6_cell(rpg, i, half_cell)?;
//         out.image.copy_from(&view, i * half_cell * 2, 0)?;
//     }
//     Ok(out)
// }
/// ```js
/// 0b0000 <- [(1, 1), (2, 1), (1, 2), (2, 2)]
/// 0b0001 <- [(4, 6), (2, 1), (1, 2), (2, 2)]
/// 0b0010 <- [(1, 1), (1, 6), (1, 2), (2, 2)]
/// 0b0011 <- [(2, 6), (3, 6), (1, 2), (2, 2)]
/// 0b0100 <- [(1, 1), (2, 1), (4, 3), (2, 2)]
/// 0b0101 <- [(4, 4), (2, 1), (4, 3), (2, 2)]
/// 0b0110 <- [(1, 1), (2, 1), (3, 4), (2, 2)]
/// 0b0111 <- [(2, 4), (3, 4), (4, 3), (2, 2)]
/// 0b1000 <- [(1, 1), (2, 1), (1, 2), (1, 3)]
/// 0b1001 <- [(4, 6), (2, 1), (1, 2), (1, 3)]
/// 0b1010 <- [(1, 1), (1, 6), (1, 2), (1, 5)]
/// 0b1011 <- [(2, 6), (3, 6), (1, 2), (1, 5)]
/// 0b1100 <- [(1, 1), (2, 1), (4, 3), (3, 3)]
/// 0b1101 <- [(4, 4), (2, 1), (4, 3), (3, 3)]
/// 0b1110 <- [(1, 1), (2, 1), (3, 4), (3, 1)]
/// 0b1111 <- [(2, 4), (3, 4), (4, 3), (3, 5)]
/// ```
fn view_rpg4x6_cell(raw: &RgbaImage, mask: u8) -> ImageResult<RgbaImage> {
    let width = raw.width() / 4;
    let height = raw.height() / 6;
    let xs = match mask {
        0b0000 => [(0, 0), (1, 0), (0, 1), (1, 1)],
        0b0001 => [(3, 5), (1, 0), (0, 1), (1, 1)],
        0b0010 => [(0, 0), (0, 5), (0, 1), (1, 1)],
        0b0011 => [(1, 5), (2, 5), (0, 1), (1, 1)],
        0b0100 => [(0, 0), (1, 0), (3, 2), (1, 1)],
        0b0101 => [(3, 3), (1, 0), (3, 4), (1, 1)],
        0b0110 => [(0, 0), (0, 5), (3, 2), (1, 1)],
        0b0111 => [(3, 1), (2, 5), (3, 4), (1, 1)],
        0b1000 => [(0, 0), (1, 0), (0, 1), (0, 2)],
        0b1001 => [(3, 5), (1, 0), (0, 1), (0, 2)],
        0b1010 => [(0, 0), (0, 3), (0, 1), (0, 4)],
        0b1011 => [(1, 5), (2, 1), (0, 1), (0, 4)],
        0b1100 => [(0, 0), (1, 0), (1, 2), (2, 2)],
        0b1101 => [(3, 3), (1, 0), (3, 0), (2, 2)],
        0b1110 => [(0, 0), (0, 3), (1, 2), (2, 0)],
        0b1111 => [(1, 3), (2, 3), (1, 4), (2, 4)],
        _ => unreachable!(),
    };
    let mut out = RgbaImage::new(width * 2, height * 2);
    for (i, (x, y)) in xs.iter().enumerate() {
        let view = raw.view(*x * width, *y * height, width, height);
        let x = (i as u32 % 2) * width;
        let y = (i as u32 / 2) * height;
        out.copy_from(&view.to_image(), x, y)?;
    }
    Ok(out)
}
