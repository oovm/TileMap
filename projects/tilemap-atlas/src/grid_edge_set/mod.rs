use image::{GenericImageView, RgbaImage, SubImage};
use crate::GridAtlas;
use image::{GenericImage, ImageResult};
use crate::utils::dimension_error;
use std::path::Path;

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

impl GridAtlas for GridEdgeAtlas {
    fn get_side(&self, l: bool, u: bool, r: bool, d: bool, n: u32) -> SubImage<&RgbaImage> {
        let s = self.cell_size();
        let i = ((l as u8) | (u as u8) << 1 | (r as u8) << 2 | (d as u8) << 3) as u32;
        let j = n % *self.count.get(i as usize).unwrap() as u32;
        self.image.view(i * s, j * s, s, s)
    }
}