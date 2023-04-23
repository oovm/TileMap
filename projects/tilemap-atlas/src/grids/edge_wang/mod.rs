use super::*;

mod as_complete;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridEdgeWang {
    key: String,
    cell_w: u32,
    cell_h: u32,
}

// constructors
impl GridEdgeWang {
    pub fn new<S>(key: S, width: u32, height: u32) -> Self
    where
        S: ToString,
    {
        Self { key: key.to_string(), cell_w: width, cell_h: height }
    }
}

// getters
impl GridEdgeWang {
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
    pub fn get_corner(&self, root: &Path, lu: bool, ru: bool, ld: bool, rd: bool) -> ImageResult<RgbaImage> {
        let mask = (lu as u8) << 0 | (ru as u8) << 1 | (ld as u8) << 2 | (rd as u8) << 3;
        let image = self.get_image(root)?;
        Ok(view_wang4x4e_cell(&image, mask).to_image())
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
