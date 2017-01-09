use super::*;
use serde::ser::SerializeStruct;

impl Serialize for FileSystemTiles {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("FileSystemTiles", 3)?;
        state.serialize_field("size", &self.size)?;
        state.end()
    }
}
