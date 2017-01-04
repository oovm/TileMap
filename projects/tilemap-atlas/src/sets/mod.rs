use dashmap::DashMap;

#[derive(Clone, Debug, Default)]
pub struct AtlasSet<G> {
    atlas: DashMap<String, G>,
    palette: DashMap<(u32, u32), AtlasReference>,
}

pub struct AtlasReference {
    name: String,
    index: usize,
    variant: usize,
}
