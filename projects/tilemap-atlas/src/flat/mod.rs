
use std::cell::OnceCell;
use image::{Rgba, RgbaImage};
pub use errors::{Error, Result};


/// Must 6 * 8 = 48
pub struct TileAtlas4x6 {
    image: RgbaImage,
    cache: [RgbaImage; 16],
}

impl TileAtlas4x6 {
    pub fn get(&self, index: usize) -> &RgbaImage {
        match self.cache.get(index) {
            Some(s) => {
                s
            }
            None => {
                panic!("index must in range `[0b0000, 0b1111]`");
            }
        }
    }
    fn cell_width(&self) -> u32 {
        self.image.width() / 4
    }
    fn cell_height(&self) -> u32 {
        self.image.height() / 6
    }
    // 			{{0,0},{1,0},{0,1},{1,1}},//0
    // 			{{3,5},{1,0},{0,1},{1,1}},//1
    // 			{{0,0},{0,5},{0,1},{1,1}},//2
    // 			{{1,5},{2,5},{0,1},{1,1}},//3
    // 			{{0,0},{1,0},{3,2},{1,1}},//4
    // 			{{3,3},{1,0},{3,4},{1,1}},//5
    // 			{{0,0},{0,5},{3,2},{1,1}},//6
    // 			{{3,1},{2,5},{3,4},{1,1}},//7
    // 			{{0,0},{1,0},{0,1},{0,2}},//8
    // 			{{3,5},{1,0},{0,1},{0,2}},//9
    // 			{{0,0},{0,3},{0,1},{0,4}},//10
    // 			{{1,5},{2,1},{0,1},{0,4}},//11
    // 			{{0,0},{1,0},{1,2},{2,2}},//12
    // 			{{3,3},{1,0},{3,0},{2,2}},//13
    // 			{{0,0},{0,3},{1,2},{2,0}},//14
    // 			{{1,3},{2,3},{1,4},{2,4}},//15
    fn make_cell(&self, x: usize, y: usize) -> RgbaImage {
        let mut image = RgbaImage::new(self.cell_width() * 2, self.cell_height() * 2);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let x = x as usize;
            let y = y as usize;
            let x = x / self.cell_width() as usize;
            let y = y / self.cell_heig
        }
    }
}


/// Must 6 * 8 = 48
pub struct TileAtlas6x8 {
    image: RgbaImage,
}