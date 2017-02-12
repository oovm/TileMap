use super::*;
use crate::{GridCompleteAtlas, GridCornerRMVX};

impl GridCornerRMXP {
    /// This conversion may causes loss of detail
    pub fn as_rpg_maker_vx(&self) -> GridCornerRMVX {
        let w = self.image.width() / 3;
        let h = self.image.height() / 4;
        let mut output = RgbaImage::new(w * 2, h * 3);
        for i in 0..2 {
            for j in 0..3 {
                let (x, y) = rpg6x8_to_rpg4x6(i, j);
                output.copy_from(&self.image, x * w, y * h).ok();
            }
        }
        unsafe { GridCornerRMVX::create(output) }
    }
    /// This conversion may causes loss of detail
    pub fn as_complete(&self) -> GridCompleteAtlas {
        self.as_rpg_maker_vx().as_complete()
    }
}

fn rpg6x8_to_rpg4x6(x: u32, y: u32) -> (u32, u32) {
    match (x, y) {
        (0, 0) => (1, 0),
        (0, 1) => (0, 1),
        (0, 2) => (0, 3),
        (1, 0) => (2, 0),
        (1, 1) => (2, 1),
        (1, 2) => (2, 3),
        _ => unreachable!(),
    }
}
