use crate::{traits::io_error, AnimationFrame, GridCornerAtlas, GridCornerRMXP, GridCornerWang, TilesProvider};

use dashmap::DashMap;
use image::{ImageResult, RgbaImage};
use serde::{Deserialize, Serialize};
use serde_json::ser::PrettyFormatter;
use std::{
    fs::{create_dir_all, File},
    io::ErrorKind,
    path::{Path, PathBuf},
};

mod der;
mod ser;

pub mod animations;
pub mod grids;

impl TilesProvider for FileSystemTiles {}

#[derive(Clone, Debug)]
pub struct FileSystemTiles {
    workspace: PathBuf,
    target_w: u32,
    target_h: u32,
    atlas: DashMap<String, TileAtlasData>,
    cache: DashMap<String, RgbaImage>,
}

impl Default for FileSystemTiles {
    fn default() -> Self {
        Self { workspace: Default::default(), target_w: 32, target_h: 32, atlas: Default::default(), cache: Default::default() }
    }
}

impl FileSystemTiles {
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
        self.target_w = size as u32;
        self.write_json()
    }
    pub fn get_cell_size(&self) -> u32 {
        self.target_w
    }
    pub fn get_atlas(&self, name: &str, _mask: u8) -> Option<TileAtlasData> {
        self.atlas.get(name).map(|a| a.value().clone())
    }
    pub fn get_corner_atlas(&self, name: &str, _mask: u8) -> Option<TileAtlasData> {
        self.atlas.get(name).map(|a| a.value().clone())
    }
    pub fn get_side_atlas(&self, name: &str, _mask: u8) -> Option<TileAtlasData> {
        self.atlas.get(name).map(|a| a.value().clone())
    }
    pub fn insert_atlas(&self, file_name: &str, _kind: TileAtlasKind) -> ImageResult<String> {
        let name = Path::new(file_name).file_stem().and_then(|s| s.to_str()).filter(|s| !s.is_empty());
        let name = match name {
            Some(name) => name.to_string(),
            None => io_error(format!("The file {:?} is not a valid image file", file_name), ErrorKind::InvalidInput)?,
        };
        // let atlas = TileAtlas::new(&self.workspace.join(file_name), &name, kind)?;
        // self.atlas.insert(name.clone(), atlas);
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
    Animation(Box<AnimationFrame>),
    GridCorner(Box<GridCornerAtlas>),
    GridCornerWang(Box<GridCornerWang>),
    GridRpgMakerXP(Box<GridCornerRMXP>),
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

impl TileAtlasData {
    pub fn get_name(&self) -> &str {
        match self {
            TileAtlasData::SimpleSet { .. } => "SimpleSet",
            TileAtlasData::Animation(_) => "Animation",
            TileAtlasData::GridCorner(v) => v.key.as_str(),
            TileAtlasData::GridCornerWang(v) => v.get_key(),
            TileAtlasData::GridRpgMakerXP(v) => v.get_key(),
            TileAtlasData::GridEdge => "GridEdge",
            TileAtlasData::GridEdgeWang => "GridEdgeWang",
        }
    }

    pub fn new(path: &Path, _name: &str, kind: TileAtlasKind) -> ImageResult<Self> {
        let _image = image::open(&path)?.to_rgba8();
        let _size = 0;
        match kind {
            TileAtlasKind::GridCorner => {
                todo!()
            }
            TileAtlasKind::GridCornerWang => {
                // let wang = GridCornerAtlas::from_wang(&image)?;
                // size = wang.cell_size();
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
        todo!()
    }
}
