use crate::{
    traits::io_error, AnimationFrame, GridCornerAtlas, GridCornerRMVXFile, GridCornerWang, GridEdgeAtlas, GridEdgeWang,
    GridSimpleAtlas, TilesProvider,
};

use crate::utils::grid_corner_mask;
use dashmap::DashMap;
use image::{ImageResult, RgbaImage};
use serde::{Deserialize, Serialize};
use serde_json::ser::PrettyFormatter;
use std::{
    fs::{create_dir_all, File},
    io::ErrorKind,
    num::NonZeroU32,
    path::{Path, PathBuf},
};

mod der;
mod ser;

impl TilesProvider for FileSystemTiles {}

#[derive(Clone, Debug)]
pub struct FileSystemTiles {
    workspace: PathBuf,
    target_w: NonZeroU32,
    target_h: NonZeroU32,
    atlas: DashMap<String, TileAtlasData>,
}

impl Default for FileSystemTiles {
    fn default() -> Self {
        unsafe {
            Self {
                workspace: Default::default(),
                target_w: NonZeroU32::new_unchecked(32),
                target_h: NonZeroU32::new_unchecked(32),
                atlas: Default::default(),
            }
        }
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
    pub fn get_target_size(&self) -> (u32, u32) {
        (self.target_w.get(), self.target_h.get())
    }
    pub fn set_target_size(&mut self, width: u32, height: u32) -> ImageResult<()> {
        match NonZeroU32::new(width) {
            Some(w) => self.target_w = w,
            None => io_error("The width of the atlas must be greater than zero", ErrorKind::InvalidInput)?,
        }
        match NonZeroU32::new(height) {
            Some(h) => self.target_h = h,
            None => io_error("The height of the atlas must be greater than zero", ErrorKind::InvalidInput)?,
        }
        self.write_json()
    }
    pub fn get_atlas(&self, name: &str, _mask: u8) -> Option<TileAtlasData> {
        self.atlas.get(name).map(|a| a.value().clone())
    }
    pub fn get_corner(&self, name: &str, lu: bool, ru: bool, ld: bool, rd: bool, index: u8) -> Option<RgbaImage> {
        let mask = grid_corner_mask(lu, ru, ld, rd);
        match self.atlas.get(name)?.value() {
            TileAtlasData::SimpleSet(_) => None,
            TileAtlasData::Animation(_) => None,
            TileAtlasData::GridCorner(v) => v.load_corner(&self.workspace, mask as u32, index as u32).ok(),
            TileAtlasData::GridCornerWang(v) => v.load_corner(&self.workspace, mask).ok(),
            TileAtlasData::GridRpgMakerVX(v) => v.load_corner(&self.workspace, mask).ok(),
            TileAtlasData::GridEdge(_) => None,
            TileAtlasData::GridEdgeWang(_) => None,
        }
    }
    pub fn get_side_atlas(&self, file: &str, _mask: u8) -> Option<TileAtlasData> {
        self.atlas.get(file).map(|a| a.value().clone())
    }
    pub fn insert_atlas(&self, file: &str, data: TileAtlasData) -> ImageResult<()> {
        self.atlas.insert(file.to_string(), data);
        self.write_json()?;
        Ok(())
    }
    pub fn update_atlas(&self, file: &str) -> ImageResult<()> {
        match self.atlas.get(file) {
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
pub enum TileAtlasKind {
    GridCorner,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(tag = "type")]
pub enum TileAtlasData {
    SimpleSet(Box<GridSimpleAtlas>),
    Animation(Box<AnimationFrame>),
    GridCorner(Box<GridCornerAtlas>),
    GridCornerWang(Box<GridCornerWang>),
    GridRpgMakerVX(Box<GridCornerRMVXFile>),
    GridEdge(Box<GridEdgeAtlas>),
    GridEdgeWang(Box<GridEdgeWang>),
}

impl TileAtlasData {
    pub fn get_name(&self) -> &str {
        match self {
            TileAtlasData::SimpleSet(v) => v.get_key(),
            TileAtlasData::Animation(v) => v.get_key(),
            TileAtlasData::GridCorner(v) => v.get_key(),
            TileAtlasData::GridCornerWang(v) => v.get_key(),
            TileAtlasData::GridRpgMakerVX(v) => v.get_key(),
            TileAtlasData::GridEdge(v) => v.get_key(),
            TileAtlasData::GridEdgeWang(v) => v.get_key(),
        }
    }
}
