use super::*;
use image::RgbaImage;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GridCornerWang {}

impl TileAtlas {
    pub fn load_grid_corner_wang(path: &Path, cell_size: u32) -> ImageResult<Self> {
        let image = image::open(&path)?.to_rgba8();
        let atlas = GridCornerAtlas::from_wang(&image)?;
        Ok(Self {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            kind: TileAtlasKind::GridCornerWang(atlas),
            cell_size,
        })
    }
}

/// ```js
/// 0b0000 <- 0  <- (1, 4)
/// 0b0001 <- 8  <- (4, 4)
/// 0b0010 <- 1  <- (1, 3)
/// 0b0011 <- 9  <- (2, 3)
/// 0b0100 <- 4  <- (1, 1)
/// 0b0101 <- 12 <- (4, 3)
/// 0b0110 <- 5  <- (3, 4)
/// 0b0111 <- 13 <- (4, 2)
/// 0b1000 <- 2  <- (2, 3)
/// 0b1001 <- 10 <- (1, 2)
/// 0b1010 <- 3  <- (2, 1)
/// 0b1011 <- 11 <- (3, 3)
/// 0b1100 <- 6  <- (4, 1)
/// 0b1101 <- 14 <- (3, 1)
/// 0b1110 <- 7  <- (2, 2)
/// 0b1111 <- 15 <- (3, 2)
/// ```
fn make_wing_cell(raw: &RgbaImage, id: u32, s: u32) -> SubImage<&RgbaImage> {
    match id {
        0b0000 => raw.view(0 * s, 3 * s, s, s),
        0b0001 => raw.view(3 * s, 3 * s, s, s),
        0b0010 => raw.view(0 * s, 2 * s, s, s),
        0b0011 => raw.view(1 * s, 2 * s, s, s),
        0b0100 => raw.view(0 * s, 0 * s, s, s),
        0b0101 => raw.view(3 * s, 2 * s, s, s),
        0b0110 => raw.view(2 * s, 3 * s, s, s),
        0b0111 => raw.view(3 * s, 1 * s, s, s),
        0b1000 => raw.view(1 * s, 3 * s, s, s),
        0b1001 => raw.view(0 * s, 1 * s, s, s),
        0b1010 => raw.view(1 * s, 0 * s, s, s),
        0b1011 => raw.view(2 * s, 2 * s, s, s),
        0b1100 => raw.view(3 * s, 0 * s, s, s),
        0b1101 => raw.view(2 * s, 0 * s, s, s),
        0b1110 => raw.view(1 * s, 1 * s, s, s),
        0b1111 => raw.view(2 * s, 1 * s, s, s),
        _ => unreachable!(),
    }
}
