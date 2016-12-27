use super::*;


impl GridCornerAtlas {
    /// Create a new tile set from rpg maker atlas.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use tilemap_atlas::TileAtlas4x6;
    /// use image::RgbaImage;
    /// ```
    pub fn from_rpg_maker_xp(image: &RgbaImage) -> Self {
        assert_eq!(image.width() % 4, 0, "image width {} does not divide by 4", image.width());
        assert_eq!(image.height() % 6, 0, "image height {} does not divide by 6", image.height());
        let mut out = Self {
            images: Default::default(),
        };
        // SAFETY: dimensions already checked
        unsafe {
            out.make_rpg4x6(image);
        }
        out
    }
    /// Make from RPG Maker XP tile set.
    unsafe fn make_rpg4x6(&mut self, raw: &RgbaImage) {
        self.images[00] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (0, 1), (1, 1)]);
        self.images[01] = self.make_rpg4x6_cell(&raw, [(3, 5), (1, 0), (0, 1), (1, 1)]);
        self.images[02] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 5), (0, 1), (1, 1)]);
        self.images[03] = self.make_rpg4x6_cell(&raw, [(1, 5), (2, 5), (0, 1), (1, 1)]);
        self.images[04] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (3, 2), (1, 1)]);
        self.images[05] = self.make_rpg4x6_cell(&raw, [(3, 3), (1, 0), (3, 4), (1, 1)]);
        self.images[06] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 5), (3, 2), (1, 1)]);
        self.images[07] = self.make_rpg4x6_cell(&raw, [(3, 1), (2, 5), (3, 4), (1, 1)]);
        self.images[08] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (0, 1), (0, 2)]);
        self.images[09] = self.make_rpg4x6_cell(&raw, [(3, 5), (1, 0), (0, 1), (0, 2)]);
        self.images[10] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 3), (0, 1), (0, 4)]);
        self.images[11] = self.make_rpg4x6_cell(&raw, [(1, 5), (2, 1), (0, 1), (0, 4)]);
        self.images[12] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (1, 2), (2, 2)]);
        self.images[13] = self.make_rpg4x6_cell(&raw, [(3, 3), (1, 0), (3, 0), (2, 2)]);
        self.images[14] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 3), (1, 2), (2, 0)]);
        self.images[15] = self.make_rpg4x6_cell(&raw, [(1, 3), (2, 3), (1, 4), (2, 4)]);
    }
    /// Make a cell from RPG Maker XP tile set.
    /// - index: [left up, right up, left down, right down]
    unsafe fn make_rpg4x6_cell(&self, raw: &RgbaImage, index: [(u32, u32); 4]) -> RgbaImage {
        let w = raw.width() / 4;
        let h = raw.height() / 6;
        let mut out = RgbaImage::new(w * 2, h * 2);
        for (k, (i, j)) in index.iter().enumerate() {
            for dx in 0..w {
                for dy in 0..h {
                    let x = (k as u32 % 2) * w + dx;
                    let y = (k as u32 / 2) * h + dy;
                    let pixel = raw.get_pixel(w * i + dx, h * j + dy);
                    out.put_pixel(x, y, *pixel);
                }
            }
        }
        out
    }
}


impl GridCornerAtlas {
    /// Create a new tile set from rpg maker atlas.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```
    /// use tilemap_atlas::TileAtlas4x6;
    /// use image::RgbaImage;
    /// ```
    pub fn from_rpg_maker_mv(image: &RgbaImage) -> Self {
        assert_eq!(image.width() % 4, 0, "image width {} does not divide by 4", image.width());
        assert_eq!(image.height() % 6, 0, "image height {} does not divide by 6", image.height());
        let mut out = Self {
            images: Default::default(),
        };
        // SAFETY: dimensions already checked
        unsafe {
            out.make_rpg4x6(image);
        }
        out
    }
    /// Make from RPG Maker MV tile set.
    unsafe fn make_rpg6x8(&mut self, raw: &RgbaImage) {
        self.images[00] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (0, 1), (1, 1)]);
        self.images[01] = self.make_rpg4x6_cell(&raw, [(3, 5), (1, 0), (0, 1), (1, 1)]);
        self.images[02] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 5), (0, 1), (1, 1)]);
        self.images[03] = self.make_rpg4x6_cell(&raw, [(1, 5), (2, 5), (0, 1), (1, 1)]);
        self.images[04] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (3, 2), (1, 1)]);
        self.images[05] = self.make_rpg4x6_cell(&raw, [(3, 3), (1, 0), (3, 4), (1, 1)]);
        self.images[06] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 5), (3, 2), (1, 1)]);
        self.images[07] = self.make_rpg4x6_cell(&raw, [(3, 1), (2, 5), (3, 4), (1, 1)]);
        self.images[08] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (0, 1), (0, 2)]);
        self.images[09] = self.make_rpg4x6_cell(&raw, [(3, 5), (1, 0), (0, 1), (0, 2)]);
        self.images[10] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 3), (0, 1), (0, 4)]);
        self.images[11] = self.make_rpg4x6_cell(&raw, [(1, 5), (2, 1), (0, 1), (0, 4)]);
        self.images[12] = self.make_rpg4x6_cell(&raw, [(0, 0), (1, 0), (1, 2), (2, 2)]);
        self.images[13] = self.make_rpg4x6_cell(&raw, [(3, 3), (1, 0), (3, 0), (2, 2)]);
        self.images[14] = self.make_rpg4x6_cell(&raw, [(0, 0), (0, 3), (1, 2), (2, 0)]);
        self.images[15] = self.make_rpg4x6_cell(&raw, [(1, 3), (2, 3), (1, 4), (2, 4)]);
    }
    /// Make a cell from RPG Maker MV tile set.
    /// - index: [left up, right up, left down, right down]
    unsafe fn make_rpg6x8_cell(&self, raw: &RgbaImage, index: [(u32, u32); 4]) -> RgbaImage {
        let w = raw.width() / 6;
        let h = raw.height() / 8;
        let mut out = RgbaImage::new(w * 2, h * 2);
        for (k, (i, j)) in index.iter().enumerate() {
            for dx in 0..w {
                for dy in 0..h {
                    let x = (k as u32 % 2) * w + dx;
                    let y = (k as u32 / 2) * h + dy;
                    let pixel = raw.get_pixel(w * i + dx, h * j + dy);
                    out.put_pixel(x, y, *pixel);
                }
            }
        }
        out
    }
}
