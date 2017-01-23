use crossbeam_skiplist::SkipMap;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::{Arc, LockResult, Mutex},
};
use tileset::{GridAtlas, TilesProvider};

/// Shared tile grid atlas.
pub struct GridTileManager<P: TilesProvider> {
    tiles: Arc<P>,
}

impl<P> GridTileManager<P>
where
    P: TilesProvider,
{
    pub fn new(provider: P) -> Self {
        Self { tiles: Arc::new(provider) }
    }
}

pub struct TileGridMap {
    map: BTreeMap<usize, TileGridMap>,
}
