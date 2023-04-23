use super::*;

/// Create a new tile set from rpg maker xp atlas.
///
/// ## Example
///
/// ```no_run
/// # use tileset::{GridAtlas, GridCompleteAtlas};
/// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
/// let size = raw.get_cell_size();
/// ```
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridSimpleAtlas {
    key: String,
    cell_w: u32,
    cell_h: u32,
    grid_w: u32,
    grid_h: u32,
}

impl GridSimpleAtlas {
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    pub fn get_key(&self) -> &str {
        &self.key
    }
}
