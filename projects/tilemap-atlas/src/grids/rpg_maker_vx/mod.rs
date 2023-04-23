use super::*;
use crate::utils::check_image_multiple;
mod to_complete;

/// A corner type tile set used in [RPG Maker VX](), [RPG MakerMV](), [RPG MakerMZ]().
///
/// ## Example
///
/// ![]()
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridCornerRMVX {
    image: RgbaImage,
}

impl GridAtlas for GridCornerRMVX {
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
    ///     GridCornerWang::create(&image, (0, 0), (image.width() / 4, image.height() / 6)).unwrap();
    /// ```
    fn create(image: &RgbaImage, (x, y): (u32, u32), (w, h): (u32, u32)) -> ImageResult<Self> {
        let (image_w, image_h) = image.dimensions();
        if x + w * 4 > image_w || y + h * 6 > image_h {
            io_error("The image size has out of range", ErrorKind::InvalidInput)?;
        }
        let view = image.view(x, y, w * 4, h * 6);
        // SAFETY: The image has been checked.
        unsafe { Ok(Self::new(view.to_image())) }
    }

    fn get_cell_size(&self) -> (u32, u32) {
        (self.image.width() / 4, self.image.height() / 6)
    }

    fn get_image(&self) -> &RgbaImage {
        &self.image
    }

    fn get_by_mask(&self, mask: u8) -> RgbaImage {
        todo!("{}", mask)
    }

    fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let image = image::open(path)?.to_rgba8();
        check_image_multiple(&image, 4, 6)?;
        Ok(Self { image })
    }
}
