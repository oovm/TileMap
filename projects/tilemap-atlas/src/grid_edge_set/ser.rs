use std::path::Path;
use image::ImageResult;
use crate::grid_edge_set::GridEdgeAtlas;

impl GridEdgeAtlas {
    pub fn save<P>(&self, path: P) -> ImageResult<()> where P: AsRef<Path> {
        self.image.save(path)
    }
}