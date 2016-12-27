use image::RgbaImage;

mod ser;
mod der;

pub struct GridEdgeAtlas {
    image: RgbaImage,
    count: [u8; 16],
}

impl GridEdgeAtlas {
    pub fn new(image: RgbaImage, count: [u8; 16]) -> Self {
        assert_eq!(image.width() % 16, 0, "image width {} does not divide by 16", image.width());
        let cell_size = image.width() / 16;
        assert_eq!(image.height() % cell_size, 0, "image height {} does not divide by cell size {}", image.height(), cell_size);
        Self {
            image,
            count,
        }
    }
    /// Create a grid edge atlas without check
    pub unsafe fn create(image: RgbaImage, count: [u8; 16]) -> Self {
        Self {
            image,
            count,
        }
    }
}

impl GridEdgeAtlas {
    pub fn cell_size(&self) -> u32 {
        self.image.width() / 16
    }
}