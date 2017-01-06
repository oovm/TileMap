use crate::TilesProvider;
use dashmap::DashMap;
use fs::create_dir_all;
use image::{ImageError, ImageResult};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Error, ErrorKind},
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
    pub fn new<S>(workspace: S, size: usize) -> ImageResult<Self>
    where
        S: AsRef<Path>,
    {
        assert_ne!(size, 0, "The size of the atlas must be greater than zero");
        let mut out = Self { workspace: PathBuf::from(workspace.as_ref()), size: size as u32, atlas: BTreeMap::new() };
        out.ensure_path()?;
        out.write_json()?;
        Ok(out)
    }
    fn ensure_path(&mut self) -> ImageResult<()> {
        create_dir_all(&self.workspace).unwrap();
        self.workspace = self.workspace.canonicalize()?;
        if !self.workspace.is_dir() {
            return Err(ImageError::IoError(Error::new(
                ErrorKind::InvalidInput,
                format!("The path {:?} is not a directory", self.workspace.display()),
            )));
        }
        Ok(())
    }
    fn write_json(&self) -> ImageResult<()> {
        let json = serde_json5::to_string(&self).unwrap();
        std::fs::write(self.workspace.join("TileSet.json5"), json).unwrap();
        Ok(())
    }

    pub fn load<S>(workspace: S) -> ImageResult<Self>
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
