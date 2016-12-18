use serde::{Deserialize, Deserializer};
use crate::TileAtlas4x6;

impl<'de> Deserialize<'de> for TileAtlas4x6 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}

struct VisitorAtlas4x6;

