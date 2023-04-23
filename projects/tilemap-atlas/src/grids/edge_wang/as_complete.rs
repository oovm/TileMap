use super::*;

impl From<GridEdgeWang> for GridCompleteAtlas {
    fn from(edge: GridEdgeWang) -> Self {
        edge.as_complete()
    }
}

impl GridEdgeWang {
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    pub fn as_complete(&self) -> GridCompleteAtlas {
        let (w, h) = self.get_cell_size();
        let mut target = RgbaImage::new(w * 12, h * 4);
        for i in 0..12 {
            for j in 0..4 {
                let (sw, sh) = edge_to_complete((i, j));
                let view = self.image.view(sw * w, sh * h, w, h);
                target.copy_from(&*view, i * w, j * h).ok();
            }
        }
        // SAFETY: The image has been checked.
        unsafe { GridCompleteAtlas::new(target) }
    }
}

fn edge_to_complete((x, y): (u32, u32)) -> (u32, u32) {
    match (x, y) {
        (c, _) if c < 4 => (x, y),
        (4, 0) => (2, 1),
        (4, 1) => (1, 1),
        (4, 2) => (1, 1),
        (4, 3) => (2, 1),
        (5, 0) => (2, 0),
        (5, 1) => (2, 1),
        (5, 2) => (2, 1),
        (5, 3) => (2, 2),
        (6, 0) => (2, 0),
        (6, 1) => (2, 1),
        (6, 2) => (2, 1),
        (6, 3) => (2, 2),
        (7, 0) => (2, 1),
        (7, 1) => (3, 1),
        (7, 2) => (3, 1),
        (7, 3) => (2, 1),
        (8, 0) => (1, 0),
        (8, 1) => (1, 1),
        (8, 2) => (2, 1),
        (8, 3) => (1, 2),
        (9, 0) => (2, 1),
        (9, 1) => (2, 1),
        (9, 2) => (2, 1),
        (9, 3) => (2, 2),
        (10, 0) => (2, 0),
        (10, 1) => (0, 3), // empty
        (10, 2) => (2, 1),
        (10, 3) => (2, 1),
        (11, 0) => (3, 0),
        (11, 1) => (2, 1),
        (11, 2) => (3, 1),
        (11, 3) => (3, 2),
        _ => unreachable!(),
    }
}
