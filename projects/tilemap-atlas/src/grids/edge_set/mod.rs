use super::*;

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
/// # use tileset::GridEdgeAtlas;
/// ```
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridEdgeAtlas {
    pub(crate) key: String,
    pub(crate) cell_w: u32,
    pub(crate) cell_h: u32,
    pub(crate) count: [u32; 16],
}

impl GridEdgeAtlas {
    pub fn get_key(&self) -> &str {
        &self.key
    }
}
