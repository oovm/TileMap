use super::*;

mod to_complete;

/// A corner type tile set used in [RPG Maker 2000](https://store.steampowered.com/app/383730), [RPG Maker 2003](https://store.steampowered.com/app/362870), [RPG Maker XP](https://store.steampowered.com/app/235900).
///
/// ## Example
///
/// ![](https://raw.githubusercontent.com/oovm/TileMap/8dfdb57648ac8ff1b3b86ab7332994812e112e4b/projects/tilemap-atlas/tests/rpg6x8/forest.png)
///
/// ![](https://raw.githubusercontent.com/oovm/TileMap/8dfdb57648ac8ff1b3b86ab7332994812e112e4b/projects/tilemap-atlas/tests/rpg6x8/forest-std.png)
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridCornerRMXP {
    image: RgbaImage,
}

impl GridAtlas for GridCornerRMXP {
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
    ///     GridCornerWang::create(&image, (0, 0), (image.width() / 6, image.height() / 8)).unwrap();
    /// ```
    fn create(image: &RgbaImage, (x, y): (u32, u32), (w, h): (u32, u32)) -> ImageResult<Self> {
        let (image_w, image_h) = image.dimensions();
        if x + w * 6 > image_w || y + h * 8 > image_h {
            io_error("The image size has out of range", ErrorKind::InvalidInput)?;
        }
        let view = image.view(x, y, w * 6, h * 8);
        // SAFETY: The image has been checked.
        unsafe { Ok(Self::new(view.to_image())) }
    }

    fn get_cell_size(&self) -> (u32, u32) {
        (self.image.width() / 6, self.image.height() / 8)
    }

    fn get_image(&self) -> &RgbaImage {
        &self.image
    }

    fn get_by_mask(&self, _: u8) -> RgbaImage {
        todo!()
    }

    fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let image = image::open(path)?.to_rgba8();
        check_image_multiple(&image, 6, 8)?;
        Ok(Self { image })
    }
}
