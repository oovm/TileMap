use super::*;
use serde::{de::Visitor, Deserializer};
use std::fmt::Formatter;

impl<'de> Deserialize<'de> for FileSystemTiles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(VisitorFileSystemTiles)
    }
}

struct VisitorFileSystemTiles;

impl<'de> Visitor<'de> for VisitorFileSystemTiles {
    type Value = FileSystemTiles;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except FileSystemTiles {size}")
    }
}
