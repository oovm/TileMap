use super::*;
use image::{GenericImage, ImageResult};
use serde::{de::Visitor, Deserialize, Deserializer};
use std::fmt::Formatter;

struct VisitorGridAtlas;

impl<'de> Deserialize<'de> for GridCornerOwned {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(VisitorGridAtlas)
    }
}

impl<'de> Visitor<'de> for VisitorGridAtlas {
    type Value = GridCornerOwned;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("except TileAtlas4x6 {width, height, image}")
    }
}

impl GridCornerOwned {
    /// Create a new tile set from rpg maker atlas.
    ///
    /// ## Panics
    ///
    /// Panics if the image width is not a multiple of 4 or the image height is not a multiple of 6.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use image::RgbaImage;
    /// use tilemap_atlas::TileAtlas4x6;
    /// ```
    pub fn from_rpg_maker_xp(rpg: &RgbaImage) -> ImageResult<Self> {
        assert_eq!(rpg.width() % 4, 0, "image width {} does not divide by 4", rpg.width());
        assert_eq!(rpg.height() % 6, 0, "image height {} does not divide by 6", rpg.height());
        let half_cell = rpg.width() / 4;
        let mut out = Self { image: RgbaImage::new(half_cell * 16 * 2, half_cell * 2), count: [1; 16] };
        for i in 0..16 {
            let view = make_rpg4x6_cell(rpg, i, half_cell)?;
            out.image.copy_from(&view, i * half_cell * 2, 0)?;
        }
        Ok(out)
    }
}

/// ```js
/// 0b0000 <- [(1, 1), (2, 1), (1, 2), (2, 2)]
/// 0b0001 <- [(4, 6), (2, 1), (1, 2), (2, 2)]
/// 0b0010 <- [(1, 1), (1, 6), (1, 2), (2, 2)]
/// 0b0011 <- [(2, 6), (3, 6), (1, 2), (2, 2)]
/// 0b0100 <- [(1, 1), (2, 1), (4, 3), (2, 2)]
/// 0b0101 <- [(4, 4), (2, 1), (4, 3), (2, 2)]
/// 0b0110 <- [(1, 1), (2, 1), (3, 4), (2, 2)]
/// 0b0111 <- [(2, 4), (3, 4), (4, 3), (2, 2)]
/// 0b1000 <- [(1, 1), (2, 1), (1, 2), (1, 3)]
/// 0b1001 <- [(4, 6), (2, 1), (1, 2), (1, 3)]
/// 0b1010 <- [(1, 1), (1, 6), (1, 2), (1, 5)]
/// 0b1011 <- [(2, 6), (3, 6), (1, 2), (1, 5)]
/// 0b1100 <- [(1, 1), (2, 1), (4, 3), (3, 3)]
/// 0b1101 <- [(4, 4), (2, 1), (4, 3), (3, 3)]
/// 0b1110 <- [(1, 1), (2, 1), (3, 4), (3, 1)]
/// 0b1111 <- [(2, 4), (3, 4), (4, 3), (3, 5)]
/// ```
fn make_rpg4x6_cell(raw: &RgbaImage, id: u32, hs: u32) -> ImageResult<RgbaImage> {
    let xs = match id {
        0b0000 => [(0, 0), (1, 0), (0, 1), (1, 1)],
        0b0001 => [(3, 5), (1, 0), (0, 1), (1, 1)],
        0b0010 => [(0, 0), (0, 5), (0, 1), (1, 1)],
        0b0011 => [(1, 5), (2, 5), (0, 1), (1, 1)],
        0b0100 => [(0, 0), (1, 0), (3, 2), (1, 1)],
        0b0101 => [(3, 3), (1, 0), (3, 4), (1, 1)],
        0b0110 => [(0, 0), (0, 5), (3, 2), (1, 1)],
        0b0111 => [(3, 1), (2, 5), (3, 4), (1, 1)],
        0b1000 => [(0, 0), (1, 0), (0, 1), (0, 2)],
        0b1001 => [(3, 5), (1, 0), (0, 1), (0, 2)],
        0b1010 => [(0, 0), (0, 3), (0, 1), (0, 4)],
        0b1011 => [(1, 5), (2, 1), (0, 1), (0, 4)],
        0b1100 => [(0, 0), (1, 0), (1, 2), (2, 2)],
        0b1101 => [(3, 3), (1, 0), (3, 0), (2, 2)],
        0b1110 => [(0, 0), (0, 3), (1, 2), (2, 0)],
        0b1111 => [(1, 3), (2, 3), (1, 4), (2, 4)],
        _ => unreachable!(),
    };
    let mut out = RgbaImage::new(hs * 2, hs * 2);
    for (i, (x, y)) in xs.iter().enumerate() {
        let view = raw.view(*x * hs, *y * hs, hs, hs);
        let x = (i as u32 % 2) * hs;
        let y = (i as u32 / 2) * hs;
        out.copy_from(&view.to_image(), x, y)?;
    }
    Ok(out)
}
