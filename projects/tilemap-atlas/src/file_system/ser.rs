use super::*;
use itertools::Itertools;
use serde::{ser::SerializeStruct, Serializer};

impl Serialize for FileSystemTiles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let items = self.atlas.iter().sorted_unstable_by(|a, b| a.key().cmp(b.key())).map(|m| m.clone()).collect::<Vec<_>>();
        let mut state = serializer.serialize_struct("FileSystemTiles", 3)?;
        state.serialize_field("atlas", &items)?;
        state.end()
    }
}
