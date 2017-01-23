/// A edge tile atlas for gridded maps
///
/// It determine the pattern of the four corners of this grid according weather four sides (left, upper, left, lower) have the same elements.
///
/// ## Load
///
/// - Standard Form
/// - RPG Maker XP
/// - RPG Maker MV
///
/// ## Examples
///
/// Suppose we have such an atlas in standard form called `atlas-std.png`;
///
/// ```no_run
/// # use tileset::TailCornerAtlas;
/// let atlas = TailCornerAtlas::load("atlas-std.png").unwrap();
/// let cell = atlas.get_side(true, true, false, true);
/// cell.save("side-1011.png").unwrap();
/// ```
pub struct GridEdgeAtlas {
    pub(crate) key: String,
    pub(crate) cell_w: u32,
    pub(crate) cell_h: u32,
    pub(crate) count: [u32; 16],
}
