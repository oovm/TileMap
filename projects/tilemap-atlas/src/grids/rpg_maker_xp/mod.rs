use super::*;

mod to_complete;

/// A corner type tile set used in [RPG Maker 2000](), [RPG Maker 2003](), [RPG Maker XP]().
///
/// ## Example
///
/// ![]()
pub struct GridCornerRMXP {
    image: RgbaImage,
    cell_w: u32,
    cell_h: u32,
}

impl GridCornerRMXP {
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::GridCornerRMXP;
    /// let raw = image::open("assets/grass-xp.png").unwrap().to_rgba8();
    /// let image = GridCornerRMXP::new(&raw, (0, 0), (raw.width() / 6, raw.height() / 8)).unwrap();
    /// ```
    pub fn new(image: &RgbaImage, (x, y): (u32, u32), (w, h): (u32, u32)) -> ImageResult<Self> {
        let max_x = x + 6 * w;
        let max_y = y + 8 * h;
        if max_x > image.width() || max_y > image.height() {
            io_error("The image size has out of range", ErrorKind::InvalidInput)?;
        }
        let view = image::imageops::crop_imm(image, x, y, w * 6, h * 8);
        // SAFETY: The image has been checked.
        unsafe { Ok(Self::create(view.to_image())) }
    }
    /// Create a new tile set from image without check.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCornerRMXP;
    /// let raw = image::open("assets/grass-xp.png").unwrap().to_rgba8();
    /// let image = unsafe { GridCornerRMXP::create(raw) };
    /// ```
    pub unsafe fn create(image: RgbaImage) -> Self {
        let cell_w = image.width() / 6;
        let cell_h = image.height() / 8;
        Self { image, cell_w, cell_h }
    }
    /// Create the tile set from supported image format, recommend use png.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCornerRMXP;
    /// let image = GridCornerRMXP::load("assets/grass-xp.png").unwrap();
    /// image.save("assets/grass-xp.png").unwrap();
    /// ```
    pub fn load<P>(path: P) -> ImageResult<Self>
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
    /// Save the tile set image to a png file, remember you need add `.png` suffix.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCornerRMXP;
    /// let image = GridCornerRMXP::load("assets/grass-xp.png").unwrap();
    /// image.save("assets/grass-xp.png").unwrap();
    /// ```
    pub fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        save_as_png(&self.image, path)
    }
}
