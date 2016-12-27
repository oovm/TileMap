use image::RgbaImage;

pub struct GridEdgeAtlas {
    image: RgbaImage,
    count: [u8; 16],
}



impl GridEdgeAtlas {
    pub fn new(image: RgbaImage, count: [u8; 16]) -> u32 {
        self.image.width() / 16
    }
    ///
    pub unsafe fn create(image: RgbaImage, count: [u8; 16]) -> GridEdgeAtlas {
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