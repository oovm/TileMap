use super::*;
mod as_complete;

/// A edge tile atlas for gridded maps
///
/// It determine the pattern of the four corners of this grid according weather four sides (left, upper, left, lower) have the same elements.
///
/// ## Load
///
/// - Standard Form
/// - RPG Maker XP
/// - RPG Maker MV
///
/// ## Examples
///
/// Suppose we have such an atlas in standard form called `atlas-std.png`;
///
/// ```no_run
/// # use tileset::GridEdgeTiny;
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct GridEdgeTiny {
    image: RgbaImage,
}
