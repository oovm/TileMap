use std::{collections::BTreeMap, sync::Arc};
use tileset::TilesProvider;

/// Shared tile grid atlas.
pub struct GridTileManager {
    tiles: Arc<dyn TilesProvider>,
}

impl GridTileManager {
    pub fn new(provider: Arc<dyn TilesProvider>) -> Self {
        Self { tiles: provider }
    }
}

pub struct TileGridMap {
    map: BTreeMap<usize, TileGridMap>,
}
