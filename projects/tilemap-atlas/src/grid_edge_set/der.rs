use image::error::LimitError;
use image::{GenericImage, GenericImageView, ImageResult, SubImage};
use rand_core::RngCore;
use crate::utils::dimension_error;
use super::*;

impl GridEdgeAtlas {
    // pub fn load<P>(path: P) -> ImageResult<Self> where P: AsRef<std::path::Path> {
    //     let image = image::open(path)?.to_rgba8();
    //     let cell_size = image.width() / 16;
    //     if image.width() % 16 != 0 || image.height() % cell_size != 0 {
    //         Err(image::ImageError::Limits(image::LimitError::from_kind(image::LimitErrorKind::DimensionError)))?
    //     }
    //     let mut count = [0; 16];
    //     for y in 0..image.height() {
    //         for x in 0..image.width() {
    //             let pixel = image.get_pixel(x, y);
    //             if pixel[3] != 0 {
    //                 let i = (x / cell_size) as usize;
    //                 count[i] += 1;
    //             }
    //         }
    //     }
    //     Ok(Self::new(image, count))
    // }
}


impl GridEdgeAtlas {
    pub fn get_side(&self, l: bool, u: bool, r: bool, d: bool, n: u32) -> SubImage<&RgbaImage> {
        let s = self.cell_size();
        let i = ((l as u8) | (u as u8) << 1 | (r as u8) << 2 | (d as u8) << 3) as u32;
        let j = n % *self.count.get(i).unwrap() as u32;
        self.image.view(i * s, j * s, s, s)
    }
    pub fn get_side_random<R>(&self, l: bool, u: bool, r: bool, d: bool, rng: &mut R) -> SubImage<&RgbaImage> where R: RngCore {
        self.get_side(l, u, r, d, rng.next_u32())
    }
}

impl GridEdgeAtlas {
    /// A 4*4
    pub fn from_wang(wang: &RgbaImage) -> ImageResult<Self> {
        let cell_size = wang.width() / 4;
        if wang.width() % 4 != 0 && wang.width() != wang.height() {
            dimension_error()?
        }
        let mut image = RgbaImage::new(cell_size * 16, cell_size);
        let mut count = [0; 16];
        for (index, cell) in count.iter_mut().enumerate() {
            let view = make_wing_cell(wang, index, cell_size);
            let x = (index % 4) as u32 * cell_size;
            let y = (index / 4) as u32 * cell_size;
            image.copy_from(&view.to_image(), x, y)?;
            *cell = 1;
        }
        // SAFETY: definitely safe
        unsafe {
            Ok(Self::create(image, count))
        }
    }
}

// 0b0000 <- 0  <- (0, 3)
// 0b0001 <- 2  <- (1, 3)
// 0b0010 <- 1  <- (0, 2)
// 0b0011 <- 3  <- (1, 2)
// 0b0100 <- 8  <- (3, 3)
// 0b0101 <- 10 <- (2, 3)
// 0b0110 <- 9  <- (3, 2)
// 0b0111 <- 11 <- (2, 2)
// 0b1000 <- 4  <- (0, 0)
// 0b1001 <- 6  <- (1, 0)
// 0b1010 <- 5  <- (0, 1)
// 0b1011 <- 7  <- (1, 1)
// 0b1100 <- 12 <- (3, 0)
// 0b1101 <- 14 <- (2, 0)
// 0b1110 <- 13 <- (3, 1)
// 0b1111 <- 15 <- (2, 1)
fn make_wing_cell(raw: &RgbaImage, id: usize, s: u32) -> SubImage<&RgbaImage> {
    match id {
        0b0000 => raw.view(0 * s, 3 * s, s, s),
        0b0001 => raw.view(1 * s, 3 * s, s, s),
        0b0010 => raw.view(0 * s, 2 * s, s, s),
        0b0011 => raw.view(1 * s, 2 * s, s, s),
        0b0100 => raw.view(3 * s, 3 * s, s, s),
        0b0101 => raw.view(2 * s, 3 * s, s, s),
        0b0110 => raw.view(3 * s, 2 * s, s, s),
        0b0111 => raw.view(2 * s, 2 * s, s, s),
        0b1000 => raw.view(0 * s, 0 * s, s, s),
        0b1001 => raw.view(1 * s, 0 * s, s, s),
        0b1010 => raw.view(0 * s, 1 * s, s, s),
        0b1011 => raw.view(1 * s, 1 * s, s, s),
        0b1100 => raw.view(3 * s, 0 * s, s, s),
        0b1101 => raw.view(2 * s, 0 * s, s, s),
        0b1110 => raw.view(3 * s, 1 * s, s, s),
        0b1111 => raw.view(2 * s, 1 * s, s, s),
        _ => unreachable!(),
    }
}