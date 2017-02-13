use super::*;
use crate::{GridCompleteAtlas, GridCornerRMVX};

impl GridCornerRMXP {
    /// Returns a new `GridCompleteAtlas` from the current `GridCornerRMVX`.
    ///
    /// This conversion may causes loss some detail
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCornerRMVX;
    /// let rpg = GridCornerRMVX::load("assets/rpg4x6.png").unwrap();
    /// rpg.as_complete().save("assets/rpg4x6-std.png").unwrap();
    /// ```
    pub fn as_rpg_maker_vx(&self) -> GridCornerRMVX {
        let w = self.cell_w * 2;
        let h = self.cell_h * 2;
        let mut output = RgbaImage::new(w * 2, h * 3);
        for i in 0..2 {
            for j in 0..3 {
                let (x, y) = rpg6x8_to_rpg4x6(i, j);
                let view = self.image.view(x * w, y * h, w, h);
                output.copy_from(&*view, i * w, j * h).ok();
            }
        }
        unsafe { GridCornerRMVX::create(output) }
    }
    /// Returns a new `GridCompleteAtlas` from the current `GridCornerRMVX`.
    ///
    /// This conversion may causes loss some detail
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use tileset::GridCornerRMVX;
    /// let rpg = GridCornerRMVX::load("assets/rpg4x6.png").unwrap();
    /// rpg.as_complete().save("assets/rpg4x6-std.png").unwrap();
    /// ```
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
