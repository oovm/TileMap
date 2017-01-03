use super::*;

pub struct GridBackgroundAtlas {
    image: RgbaImage,
}

impl GridBackgroundAtlas {
    pub fn new(image: RgbaImage) -> Self {
        assert_eq!(image.width(), image.height(), "The width and height of the image must be equal");
        Self { image }
    }
    pub unsafe fn create(image: RgbaImage) -> Self {
        Self { image }
    }
}

impl GridAtlas for GridBackgroundAtlas {
    fn cell_size(&self) -> u32 {
        self.image.width()
    }

    fn get_cell(&self, _: bool, _: bool, _: bool, _: bool, _: u32) -> SubImage<&RgbaImage> {
        self.image.view(0, 0, self.image.width(), self.image.height())
    }
}
