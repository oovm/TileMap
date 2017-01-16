// use serde::ser::SerializeStruct;
// use serde::Serialize;
use super::*;

// impl Serialize for TailCornerAtlas {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
//         let mut state = serializer.serialize_struct("TileAtlas4x6", 1)?;
//         state.serialize_field("width", &self.image.width())?;
//         state.serialize_field("height", &self.image.height())?;
//         state.serialize_field("image", self.image.as_raw())?;
//         state.end()
//     }
// }

impl GridCornerOwned {
    pub fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        self.image.save(path)
    }
}
