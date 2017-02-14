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
    /// ```no_run
    /// # use tileset::GridCornerRMVX;
    /// let raw = image::open("assets/grass-vx.png").unwrap().to_rgba8();
    /// let image = GridCornerRMVX::new(&raw, (0, 0), (raw.width() / 4, raw.height() / 6)).unwrap();
    /// ```
    pub fn new(image: &RgbaImage, (x, y): (u32, u32), (w, h): (u32, u32)) -> ImageResult<Self> {
        let max_x = x + 4 * w;
        let max_y = y + 6 * h;
        if max_x > image.width() || max_y > image.height() {
            io_error("The image size has out of range", ErrorKind::InvalidInput)?;
        }
        let view = image::imageops::crop_imm(image, x, y, w * 4, h * 6);
        // SAFETY: The image has been checked.
        unsafe { Ok(Self::create(view.to_image())) }
    }
    /// Create a new [`GridCornerRMVX`] tile set without check.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCornerRMVX;
    /// let raw = image::open("assets/grass-vx.png").unwrap().to_rgba8();
    /// let image = unsafe { GridCornerRMVX::create(raw) };
    /// ```
    pub unsafe fn create(image: RgbaImage) -> Self {
        let cell_w = image.width() / 4;
        let cell_h = image.height() / 6;
        Self { image, cell_w, cell_h }
    }
    /// Create the tile set from supported image format, recommend use png.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCornerRMVX;
    /// let image = GridCornerRMVX::load("assets/grass-vx.png").unwrap();
    /// image.save("assets/grass-vx.png").unwrap();
    /// ```
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
    /// Save the tile set image to a png file, remember you need add `.png` suffix.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCornerRMVX;
    /// let image = GridCornerRMVX::load("assets/grass-vx.png").unwrap();
    /// image.save("assets/grass-vx.png").unwrap();
    /// ```
    pub fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        save_as_png(&self.image, path)
    }
}
