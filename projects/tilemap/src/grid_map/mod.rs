use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::{Arc, LockResult, Mutex};
use crossbeam_skiplist::SkipMap;

/// Shared tile grid atlas.
pub struct TileGridAtlas {
    atlas: Arc<Mutex<VecDeque<TileGridAtlas>>>,
}

impl Default for TileGridAtlas {
    fn default() -> Self {
        Self {
            atlas: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
}

impl TileGridAtlas {
    pub fn new(capacity: usize) -> Self {
        Self {
            atlas: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
        }
    }
    pub fn insert(&self, order: usize, value: TileGridAtlas) -> bool {
        match self.atlas.lock() {
            Ok(mut o) => {
                o.insert(order, value);
                true
            }
            Err(e) => {
                tracing::error!("Failed to lock atlas: {}", e);
                false
            }
        }
    }
    pub fn delete(&self, order: usize) -> bool {
        match self.atlas.lock() {
            Ok(mut o) => {
                o.remove(order);
                true
            }
            Err(e) => {
                tracing::error!("Failed to lock atlas: {}", e);
                false
            }
        }
    }
}


pub struct TileGridMap {
    map: BTreeMap<usize, TileGridMap>,
}