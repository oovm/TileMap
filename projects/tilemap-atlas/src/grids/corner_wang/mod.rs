use super::*;
use image::GenericImage;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridCornerWang {
    key: String,
    cell_w: u32,
    cell_h: u32,
}

// constructors
impl GridCornerWang {
    pub fn new<S>(key: S, width: u32, height: u32) -> Self
    where
        S: ToString,
    {
        Self { key: key.to_string(), cell_w: width, cell_h: height }
    }
    pub fn as_standard<S, G>(&self, name: &str, image: &RgbaImage) -> ImageResult<(GridCornerAtlas, RgbaImage)>
    where
        S: ToString,
        G: GenericImageView,
    {
        let mut output = RgbaImage::new(self.cell_w * 16, self.cell_h);
        for i in 0..16 {
            let view = view_wang4x4c_cell(image, i as u8);
            output.copy_from(&*view, i * self.cell_w, 0)?;
        }
        Ok((GridCornerAtlas { key: name.to_string(), cell_w: self.cell_w, cell_h: self.cell_h, count: [1; 16] }, output))
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
    pub fn get_key(&self) -> &str {
        &self.key
    }
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
    pub fn get_path(&self, root: &Path) -> PathBuf {
        root.join(&self.key)
    }
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
    pub fn get_image(&self, root: &Path) -> ImageResult<RgbaImage> {
        Ok(image::open(self.get_path(root))?.to_rgba8())
    }
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
    pub fn load_image(&self, root: &Path, lu: bool, ru: bool, ld: bool, rd: bool) -> ImageResult<RgbaImage> {
        let mask = (lu as u8) << 0 | (ru as u8) << 1 | (ld as u8) << 2 | (rd as u8) << 3;
        // SAFETY: mask always <= 0b1111
        unsafe { self.load_corner_by_mask(root, mask) }
    }
    pub unsafe fn load_corner_by_mask(&self, root: &Path, mask: u8) -> ImageResult<RgbaImage> {
        let image = self.get_image(root)?;
        Ok(view_wang4x4c_cell(&image, mask).to_image())
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
fn view_wang4x4c_cell(r: &RgbaImage, mask: u8) -> SubImage<&RgbaImage> {
    let w = r.width() / 4;
    let h = r.height() / 4;
    match mask {
        0b0000 => r.view(0 * w, 3 * h, w, h),
        0b0001 => r.view(3 * w, 3 * h, w, h),
        0b0010 => r.view(0 * w, 2 * h, w, h),
        0b0011 => r.view(1 * w, 2 * h, w, h),
        0b0100 => r.view(0 * w, 0 * h, w, h),
        0b0101 => r.view(3 * w, 2 * h, w, h),
        0b0110 => r.view(2 * w, 3 * h, w, h),
        0b0111 => r.view(3 * w, 1 * h, w, h),
        0b1000 => r.view(1 * w, 3 * h, w, h),
        0b1001 => r.view(0 * w, 1 * h, w, h),
        0b1010 => r.view(1 * w, 0 * h, w, h),
        0b1011 => r.view(2 * w, 2 * h, w, h),
        0b1100 => r.view(3 * w, 0 * h, w, h),
        0b1101 => r.view(2 * w, 0 * h, w, h),
        0b1110 => r.view(1 * w, 1 * h, w, h),
        0b1111 => r.view(2 * w, 1 * h, w, h),
        _ => unreachable!(),
    }
}
