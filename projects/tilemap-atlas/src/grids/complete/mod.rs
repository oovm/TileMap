use super::*;

mod convert;

/// Create a complete tile set from image.
///
/// # Examples
///
/// ```no_run
/// # use tileset::{GridAtlas, GridCompleteAtlas};
/// ```
#[derive(Clone)]
pub struct GridCompleteAtlas {
    image: RgbaImage,
    cell_w: u32,
    cell_h: u32,
}

impl GridAtlas for GridCompleteAtlas {
    unsafe fn new(image: RgbaImage) -> Self {
        let cell_w = image.width() / 12;
        let cell_h = image.height() / 4;
        Self { image, cell_w, cell_h }
    }

    /// Create a complete tile set without check.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCornerWang};
    /// let image = image::open("assets/standard/grass.png").unwrap().to_rgba8();
    /// let tile_set =
    ///     GridCornerWang::create(&image, (0, 0), (image.width() / 12, image.height() / 4)).unwrap();
    /// ```
    fn create(image: &RgbaImage, (x, y): (u32, u32), (w, h): (u32, u32)) -> ImageResult<Self> {
        let (image_w, image_h) = image.dimensions();
        if x + w * 12 > image_w || y + h * 4 > image_h {
            io_error("The image size has out of range", ErrorKind::InvalidInput)?;
        }
        let view = image.view(x, y, w * 12, h * 4);
        // SAFETY: The image has been checked.
        unsafe { Ok(Self::new(view.to_image())) }
    }

    fn get_cell_size(&self) -> (u32, u32) {
        (self.cell_w, self.cell_h)
    }

    fn get_image(&self) -> &RgbaImage {
        &self.image
    }

    fn get_by_mask(&self, mask: u8) -> RgbaImage {
        let (i, j) = convert::complete_sub_image(mask);
        let view = self.image.view(i * self.cell_w, j * self.cell_h, self.cell_w, self.cell_h);
        view.to_image()
    }

    fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let image = image::open(path)?.to_rgba8();
        Self::create(&image, (0, 0), (image.width() / 12, image.height() / 4))
    }
}
