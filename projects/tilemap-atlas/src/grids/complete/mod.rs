use super::*;

mod convert;

pub struct GridCompleteAtlas {
    image: RgbaImage,
    cell_w: u32,
    cell_h: u32,
}

impl GridCompleteAtlas {
    /// Create a new [`GridCompleteAtlas`] tile set from rpg maker atlas.
    ///
    /// # Arguments
    ///
    /// * `image`:
    ///
    /// returns: Result<GridCompleteAtlas, ImageError>
    ///
    /// # Examples
    ///
    /// ```
    /// # use tileset::GridCompleteAtlas;
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
    /// Create a new [`GridCompleteAtlas`] tile set from rpg maker atlas.
    ///
    /// # Arguments
    ///
    /// * `image`:
    ///
    /// returns: Result<GridCompleteAtlas, ImageError>
    ///
    /// # Examples
    ///
    /// ```
    /// # use tileset::GridCompleteAtlas;
    /// ```
    pub unsafe fn create(image: RgbaImage) -> Self {
        let cell_w = image.width() / 12;
        let cell_h = image.height() / 4;
        Self { image, cell_w, cell_h }
    }
    pub fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        Self::new(image::open(path)?.to_rgba8())
    }
    pub fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        self.image.save(path)
    }
}
