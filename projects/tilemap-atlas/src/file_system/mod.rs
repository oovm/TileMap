use crate::{utils::io_error, AnimationFrame, GridAtlas, GridCornerWang, GridEdgeWang, GridSimpleAtlas, TilesProvider};

use crate::utils::grid_corner_mask;
use dashmap::DashMap;
use image::{ImageResult, RgbaImage};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

/// Create a new tile set from rpg maker xp atlas.
///
/// ## Example
///
/// ```no_run
/// # use tileset::{GridAtlas, GridCompleteAtlas};
/// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
/// let size = raw.get_cell_size();
/// ```
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
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    pub fn get_target_size(&self) -> (u32, u32) {
        (self.target_w.get(), self.target_h.get())
    }
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
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
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    pub fn get_atlas(&self, name: &str, _mask: u8) -> Option<TileAtlasData> {
        self.atlas.get(name).map(|a| a.value().clone())
    }
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    pub fn get_corner(&self, name: &str, lu: bool, ru: bool, ld: bool, rd: bool, _: u8) -> Option<RgbaImage> {
        let mask = grid_corner_mask(lu, ru, ld, rd);
        match self.atlas.get(name)?.value() {
            TileAtlasData::SimpleSet(_) => None,
            TileAtlasData::Animation(_) => None,
            TileAtlasData::GridCornerWang(v) => Some(v.get_by_mask(mask)),
            TileAtlasData::GridEdgeWang(_) => None,
        }
    }
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    pub fn get_side_atlas(&self, file: &str, _mask: u8) -> Option<TileAtlasData> {
        self.atlas.get(file).map(|a| a.value().clone())
    }
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    pub fn insert_atlas(&self, file: &str, data: TileAtlasData) -> ImageResult<()> {
        self.atlas.insert(file.to_string(), data);
        self.write_json()?;
        Ok(())
    }
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
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

/// Create a new tile set from rpg maker xp atlas.
///
/// ## Example
///
/// ```no_run
/// # use tileset::{GridAtlas, GridCompleteAtlas};
/// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
/// let size = raw.get_cell_size();
/// ```
#[derive(Clone, Debug)]
pub enum TileAtlasData {
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    SimpleSet(Box<GridSimpleAtlas>),
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    Animation(Box<AnimationFrame>),
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    GridCornerWang(Box<GridCornerWang>),
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    GridEdgeWang(Box<GridEdgeWang>),
}

impl Serialize for TileAtlasData {
    fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}
impl<'de> Deserialize<'de> for TileAtlasData {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl TileAtlasData {
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    pub fn get_name(&self) -> &str {
        match self {
            TileAtlasData::SimpleSet(v) => v.get_key(),
            TileAtlasData::Animation(v) => v.get_key(),
            TileAtlasData::GridCornerWang(_) => todo!(),
            TileAtlasData::GridEdgeWang(_) => todo!(),
        }
    }
}
