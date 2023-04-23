use super::*;

impl From<GridCornerRMXP> for GridCompleteAtlas {
    fn from(rpg: GridCornerRMXP) -> Self {
        rpg.as_complete()
    }
}

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
        let w = self.get_cell_size().0 * 2;
        let h = self.get_cell_size().1 * 2;
        let mut output = RgbaImage::new(w * 2, h * 3);
        for i in 0..2 {
            for j in 0..3 {
                let (x, y) = rpg6x8_to_rpg4x6(i, j);
                let view = self.image.view(x * w, y * h, w, h);
                output.copy_from(&*view, i * w, j * h).ok();
            }
        }
        unsafe { GridCornerRMVX::new(output) }
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

/// ```js
/// 0b0000 <- [(1, 1), (2, 1), (1, 2), (2, 2)]
/// 0b0001 <- [(4, 6), (2, 1), (1, 2), (2, 2)]
/// 0b0010 <- [(1, 1), (1, 6), (1, 2), (2, 2)]
/// 0b0011 <- [(2, 6), (3, 6), (1, 2), (2, 2)]
/// 0b0100 <- [(1, 1), (2, 1), (4, 3), (2, 2)]
/// 0b0101 <- [(4, 4), (2, 1), (4, 3), (2, 2)]
/// 0b0110 <- [(1, 1), (2, 1), (3, 4), (2, 2)]
/// 0b0111 <- [(2, 4), (3, 4), (4, 3), (2, 2)]
/// 0b1000 <- [(1, 1), (2, 1), (1, 2), (1, 3)]
/// 0b1001 <- [(4, 6), (2, 1), (1, 2), (1, 3)]
/// 0b1010 <- [(1, 1), (1, 6), (1, 2), (1, 5)]
/// 0b1011 <- [(2, 6), (3, 6), (1, 2), (1, 5)]
/// 0b1100 <- [(1, 1), (2, 1), (4, 3), (3, 3)]
/// 0b1101 <- [(4, 4), (2, 1), (4, 3), (3, 3)]
/// 0b1110 <- [(1, 1), (2, 1), (3, 4), (3, 1)]
/// 0b1111 <- [(2, 4), (3, 4), (4, 3), (3, 5)]
/// ```
#[allow(unused)]
fn rpg4x6_to_wang(raw: &RgbaImage, mask: u8) -> ImageResult<RgbaImage> {
    let width = raw.width() / 4;
    let height = raw.height() / 6;
    let xs = match mask {
        0b0000 => [(0, 0), (1, 0), (0, 1), (1, 1)],
        0b0001 => [(3, 5), (1, 0), (0, 1), (1, 1)],
        0b0010 => [(0, 0), (0, 5), (0, 1), (1, 1)],
        0b0011 => [(1, 5), (2, 5), (0, 1), (1, 1)],
        0b0100 => [(0, 0), (1, 0), (3, 2), (1, 1)],
        0b0101 => [(3, 3), (1, 0), (3, 4), (1, 1)],
        0b0110 => [(0, 0), (0, 5), (3, 2), (1, 1)],
        0b0111 => [(3, 1), (2, 5), (3, 4), (1, 1)],
        0b1000 => [(0, 0), (1, 0), (0, 1), (0, 2)],
        0b1001 => [(3, 5), (1, 0), (0, 1), (0, 2)],
        0b1010 => [(0, 0), (0, 3), (0, 1), (0, 4)],
        0b1011 => [(1, 5), (2, 1), (0, 1), (0, 4)],
        0b1100 => [(0, 0), (1, 0), (1, 2), (2, 2)],
        0b1101 => [(3, 3), (1, 0), (3, 0), (2, 2)],
        0b1110 => [(0, 0), (0, 3), (1, 2), (2, 0)],
        0b1111 => [(1, 3), (2, 3), (1, 4), (2, 4)],
        _ => unreachable!(),
    };
    let mut out = RgbaImage::new(width * 2, height * 2);
    for (i, (x, y)) in xs.iter().enumerate() {
        let view = raw.view(*x * width, *y * height, width, height);
        let x = (i as u32 % 2) * width;
        let y = (i as u32 / 2) * height;
        out.copy_from(&view.to_image(), x, y)?;
    }
    Ok(out)
}
