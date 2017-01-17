use super::*;

impl GridEdgeOwned {
    pub fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        self.image.save(path)
    }
}
