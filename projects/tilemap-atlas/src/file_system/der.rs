use super::*;
use serde::{
    de::{MapAccess, Visitor},
    Deserializer,
};
use std::fmt::Formatter;

struct VisitorFileSystemTiles<'i> {
    ptr: &'i mut FileSystemTiles,
}

impl FileSystemTiles {
    pub fn load<S>(workspace: S) -> ImageResult<Self>
    where
        S: AsRef<Path>,
    {
        let path = workspace.as_ref().join("TileSet.json5");
        let json = std::fs::read_to_string(&path)?;
        match serde_json::from_str(&json) {
            Ok(out) => Ok(out),
            Err(e) => {
                io_error(format!("The file {:?} is not a valid TileSet.json5 file: {}", json, e), ErrorKind::InvalidInput)
            }
        }
    }
}

impl<'de> Deserialize<'de> for FileSystemTiles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut out = FileSystemTiles { workspace: Default::default(), size: 0, atlas: Default::default() };
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
                "size" => self.ptr.size = map.next_value()?,
                _ => {
                    map.next_value::<serde_json::Value>()?;
                }
            }
        }
        Ok(())
    }
}
