use crate::{traits::io_error, GridAtlas, GridCornerAtlas, TilesProvider};

use dashmap::DashMap;
use image::ImageResult;
use serde::{Deserialize, Serialize};
use serde_json::ser::PrettyFormatter;
use std::{
    collections::BTreeMap,
    fs::{create_dir_all, File},
    io::ErrorKind,
    path::{Path, PathBuf},
};

mod der;
mod ser;

mod grids;

pub use self::grids::corner_wang::GridCornerWang;

impl TilesProvider for FileSystemTiles {}

#[derive(Clone, Debug, Default)]
pub struct FileSystemTiles {
    workspace: PathBuf,
    size: u32,
    atlas: DashMap<String, TileAtlas>,
}

impl FileSystemTiles {
    pub fn new<S>(workspace: S, size: usize) -> ImageResult<Self>
    where
        S: AsRef<Path>,
    {
        assert_ne!(size, 0, "The size of the atlas must be greater than zero");
        let mut out = Self { workspace: PathBuf::from(workspace.as_ref()), size: size as u32, atlas: DashMap::new() };
        out.ensure_path()?;
        out.write_json()?;
        Ok(out)
    }
    fn ensure_path(&mut self) -> ImageResult<()> {
        create_dir_all(&self.workspace)?;
        self.workspace = self.workspace.canonicalize()?;
        if !self.workspace.is_dir() {
            io_error(format!("The path {:?} is not a directory", self.workspace.display()), ErrorKind::InvalidInput)?
        }
        Ok(())
    }
    fn write_json(&self) -> ImageResult<()> {
        let path = File::create(self.workspace.join("TileSet.json5"))?;
        let mut pretty = serde_json::Serializer::with_formatter(path, PrettyFormatter::with_indent(b"    "));
        match self.serialize(&mut pretty) {
            Ok(_) => Ok(()),
            Err(e) => io_error(
                format!("The file {:?} is not a valid TileSet.json5 file: {}", self.workspace.display(), e),
                ErrorKind::InvalidInput,
            ),
        }
    }

    pub fn set_cell_size(&mut self, size: usize) -> ImageResult<()> {
        assert_ne!(size, 0, "The size of the atlas must be greater than zero");
        self.size = size as u32;
        self.write_json()
    }
    pub fn get_cell_size(&self) -> u32 {
        self.size
    }
    pub fn get_atlas(&self, name: &str, mask: u8) -> Option<TileAtlas> {
        self.atlas.get(name).map(|a| a.value().clone())
    }
    pub fn get_corner_atlas(&self, name: &str, mask: u8) -> Option<TileAtlas> {
        self.atlas.get(name).map(|a| a.value().clone())
    }
    pub fn get_side_atlas(&self, name: &str, mask: u8) -> Option<TileAtlas> {
        self.atlas.get(name).map(|a| a.value().clone())
    }
    pub fn insert_atlas(&self, file_name: &str, kind: TileAtlasKind) -> ImageResult<String> {
        let name = Path::new(file_name).file_stem().and_then(|s| s.to_str()).filter(|s| !s.is_empty());
        let name = match name {
            Some(name) => name.to_string(),
            None => io_error(format!("The file {:?} is not a valid image file", file_name), ErrorKind::InvalidInput)?,
        };
        let atlas = TileAtlas::new(&self.workspace.join(file_name), &name, kind)?;
        self.atlas.insert(name.clone(), atlas);
        self.write_json()?;
        Ok(name)
    }
    pub fn update_atlas(&self, name: &str) -> ImageResult<()> {
        match self.atlas.get(name) {
            Some(_) => {
                todo!()
            }
            None => {
                todo!()
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TileAtlas {
    /// Tile atlas name
    name: String,
    /// Relative path to the file
    file: String,
    /// Size of the cell in pixels
    cell_w: u32,
    cell_h: u32,
    ///
    custom: TileAtlasData,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(tag = "type")]
pub enum TileAtlasData {
    SimpleSet {
        /// The number of horizontal sprites
        columns: usize,
        /// The number of vertical sprites
        rows: usize,
        /// The number of sprites
        count: usize,
    },
    Animation {
        /// The number of horizontal sprites
        columns: usize,
        /// The number of vertical sprites
        rows: usize,
        /// The number of sprites
        count: usize,
    },
    GridCorner {
        w: usize,
        h: usize,
        mask: u8,
    },
    GridCornerWang,
    GridRpgMakerXP,
    GridEdge,
    GridEdgeWang,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TileAtlasKind {
    GridCorner,
    GridCornerWang,
    GridRpgMakerXP,
    GridEdge,
    GridEdgeWang,
}

impl Default for TileAtlasData {
    fn default() -> Self {
        Self::SimpleSet { columns: 1, rows: 1, count: 1 }
    }
}

impl Default for TileAtlasKind {
    fn default() -> Self {
        Self::GridCorner
    }
}

impl TileAtlas {
    pub fn new(path: &Path, name: &str, kind: TileAtlasKind) -> ImageResult<Self> {
        let image = image::open(&path)?.to_rgba8();
        let mut size = 0;
        match kind {
            TileAtlasKind::GridCorner => {
                todo!()
            }
            TileAtlasKind::GridCornerWang => {
                let wang = GridCornerAtlas::from_wang(&image)?;
                size = wang.cell_size();
            }
            TileAtlasKind::GridRpgMakerXP => {
                todo!()
            }
            TileAtlasKind::GridEdge => {
                todo!()
            }
            TileAtlasKind::GridEdgeWang => {
                todo!()
            }
        }
        Ok(Self { name: name.to_string(), file: name.to_string(), cell_w: 32, cell_h: 32, custom: Default::default() })
    }
}
