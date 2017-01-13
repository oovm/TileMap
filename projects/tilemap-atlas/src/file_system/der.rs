use super::*;
use serde::{
    de::{MapAccess, Visitor},
    Deserializer,
};
use serde_json::Error;
use std::fmt::Formatter;

impl FileSystemTiles {
    pub fn new<S>(workspace: S, w: usize, h: usize) -> ImageResult<Self>
    where
        S: AsRef<Path>,
    {
        assert_ne!(w, 0, "The width of the atlas must be greater than zero");
        assert_ne!(h, 0, "The height of the atlas must be greater than zero");
        let mut out = Self {
            // check path later
            workspace: PathBuf::from(workspace.as_ref()),
            size_w: w as u32,
            size_h: h as u32,
            ..Default::default()
        };
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

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "size" => self.ptr.size_w = map.next_value()?,
                _ => {
                    map.next_value::<serde_json::Value>()?;
                }
            }
        }
        Ok(())
    }
}
