use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Default)]
pub struct AtlasSets<G> {
    atlas: DashMap<String, G>,
    palette: DashMap<(u32, u32), AtlasReference>,
}

#[derive(Clone, Debug, Default)]
pub struct AtlasReference {
    name: String,
    index: usize,
    variant: usize,
}

impl TilesProvider for FileSystemTiles {}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FileSystemTiles {
    workspace: PathBuf,
    size: u32,
    atlas: BTreeMap<String, TileAtlas>,
}

impl FileSystemTiles {
    pub fn new<S>(workspace: S, size: usize) -> Self
    where
        S: AsRef<Path>,
    {
        assert_ne!(size, 0, "The size of the atlas must be greater than zero");
        let out = Self { workspace: PathBuf::from(workspace.as_ref()), size: size as u32, atlas: BTreeMap::new() };
        let json = serde_json5::to_string_pretty(&out).unwrap();
        std::fs::write(out.workspace.join("TileSet.json5"), json).unwrap();
        out
    }
    pub fn load<S>(workspace: S) -> Self
    where
        S: AsRef<Path>,
    {
        todo!()
    }

    pub fn set_cell_size(&mut self, size: usize) {
        assert_ne!(size, 0, "The size of the atlas must be greater than zero");
        self.size = size as u32;
    }
    pub fn get_cell_size(&self) -> u32 {
        self.size
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TileAtlas {
    kind: TileAtlasKind,
    size: u32,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TileAtlasKind {
    GridCorner,
    GridCornerWang,
    GridRMXP,
    GridEdge,
    GridEdgeWang,
}

impl Default for TileAtlasKind {
    fn default() -> Self {
        Self::GridCorner
    }
}
