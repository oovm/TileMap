use super::*;

mod convert;

/// Create a complete tile set from image.
///
/// # Examples
///
/// ![](https://raw.githubusercontent.com/oovm/TileMap/dev/projects/tilemap-atlas/assets/documents/1x/complete-case.png)
///
///
///
/// ![](https://raw.githubusercontent.com/oovm/TileMap/dev/projects/tilemap-atlas/assets/documents/1x/complete-atlas.png)
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridCompleteAtlas {
    image: RgbaImage,
}

impl GridAtlas for GridCompleteAtlas {
    unsafe fn new(image: RgbaImage) -> Self {
        Self { image }
    }

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
        (self.image.width() / 12, self.image.height() / 4)
    }

    fn get_image(&self) -> &RgbaImage {
        &self.image
    }

    fn get_by_mask(&self, mask: u8) -> RgbaImage {
        let (w, h) = self.get_cell_size();
        let (i, j) = convert::complete_sub_image(mask);
        let view = self.image.view(i * w, j * h, w, h);
        view.to_image()
    }

    fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let image = image::open(path)?.to_rgba8();
        check_image_multiple(&image, 12, 4)?;
        Ok(Self { image })
    }
}
