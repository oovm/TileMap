use std::fmt::Formatter;
use serde::{Deserialize, Deserializer};
use serde::de::Visitor;
use super::*;

struct VisitorAtlas4x6;

impl<'de> Deserialize<'de> for TileCornerSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_map(VisitorAtlas4x6)
    }
}


impl<'de> Visitor<'de> for VisitorAtlas4x6 {
    type Value = TileCornerSet;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except TileAtlas4x6 {width, height, image}")
    }
}

