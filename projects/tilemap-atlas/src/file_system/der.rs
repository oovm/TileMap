use super::*;
use serde::{
    de::{MapAccess, Visitor},
    Deserializer,
};

use std::fmt::Formatter;

impl FileSystemTiles {
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    pub fn new<S>(workspace: S, width: u32, height: u32) -> ImageResult<Self>
    where
        S: AsRef<Path>,
    {
        let mut out = Self {
            // check path later
            workspace: PathBuf::from(workspace.as_ref()),
            ..Default::default()
        };
        out.ensure_path()?;
        // include write json
        out.set_target_size(width, height)?;
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
    /// Create a new tile set from rpg maker xp atlas.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// # use tileset::{GridAtlas, GridCompleteAtlas};
    /// let raw: GridCompleteAtlas = GridAtlas::load("assets/grass-xp.png").unwrap();
    /// let size = raw.get_cell_size();
    /// ```
    pub fn load<S>(workspace: S) -> ImageResult<Self>
    where
        S: AsRef<Path>,
    {
        let mut out = Self { workspace: workspace.as_ref().canonicalize()?, ..Default::default() };
        let json = File::open(out.workspace.join("TileSet.json5"))?;
        let mut der = serde_json::Deserializer::from_reader(&json);
        match FileSystemTiles::deserialize_in_place(&mut der, &mut out) {
            Ok(_) => Ok(out),
            Err(e) => {
                io_error(format!("The file {:?} is not a valid TileSet.json5 file: {}", json, e), ErrorKind::InvalidInput)
            }
        }
    }
}

struct VisitorFileSystemTiles<'i> {
    ptr: &'i mut FileSystemTiles,
}

impl<'de> Deserialize<'de> for FileSystemTiles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut out = FileSystemTiles::default();
        deserializer.deserialize_map(VisitorFileSystemTiles { ptr: &mut out })?;
        Ok(out)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(VisitorFileSystemTiles { ptr: place })
    }
}

impl<'i, 'de> Visitor<'de> for VisitorFileSystemTiles<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except FileSystemTiles {size}")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                // "size" => self.ptr.target_w = map.next_value()?,
                _ => {
                    map.next_value::<serde_json::Value>()?;
                }
            }
        }
        Ok(())
    }
}
