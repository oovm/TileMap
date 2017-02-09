use super::*;

mod convert;

/// Create a complete tile set from image.
///
/// # Examples
///
/// ```no_run
/// # use tileset::GridCompleteAtlas;
/// let image = image::open("assets/standard/grass.png").unwrap().to_rgba8();
/// let tile_set = GridCompleteAtlas::new(image).unwrap();
/// ```
pub struct GridCompleteAtlas {
    image: RgbaImage,
    cell_w: u32,
    cell_h: u32,
}

impl GridCompleteAtlas {
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
    /// let image = GridCompleteAtlas::load("assets/standard/grass.png").unwrap();
    /// image.save("assets/standard/grass.png").unwrap();
    /// ```
    pub fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        save_as_png(&self.image, path)
    }
}
