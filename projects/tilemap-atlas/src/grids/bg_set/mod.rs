use super::*;

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
    pub fn get_key(&self) -> &str {
        &self.key
    }
}
