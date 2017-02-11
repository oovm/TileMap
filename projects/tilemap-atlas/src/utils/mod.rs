use crate::{GridCompleteAtlas, GridCornerRMVX};
use image::{ColorType, GenericImageView, ImageFormat, ImageResult, RgbaImage};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

pub fn decompose_image_grid_by_cells<P>(path: P, cols: u32, rows: u32) -> ImageResult<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().canonicalize()?;
    let dir = path.parent().expect("The path must have a parent directory");
    let name = path.file_stem().expect("The path must have a file name");
    let image = image::open(&path)?;
    let (width, height) = image.dimensions();
    let cell_width = width / cols;
    let cell_height = height / rows;
    for row in 0..rows {
        for col in 0..cols {
            let view = image.view(col * cell_width, row * cell_height, cell_width, cell_height);
            let out = dir.join(format!("{}-{}-{}.png", name.to_str().unwrap(), col, row));
            view.to_image().save(&out)?;
        }
    }
    Ok(())
}

pub struct AnimationSlice {}

pub fn grid_corner_mask(lu: bool, ru: bool, ld: bool, rd: bool) -> u8 {
    (lu as u8) << 0 | (ru as u8) << 1 | (ld as u8) << 2 | (rd as u8) << 3
}

#[derive(Debug)]
pub struct MaskBuilder {
    map: BTreeMap<u8, (u32, u32)>,
    defaults: (u32, u32),
}
impl Display for MaskBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "match mask {{")?;
        for (mask, (x, y)) in self.map.iter() {
            writeln!(f, "    0b{:08b} => ({}, {}),", mask, x.saturating_sub(1), y.saturating_sub(1))?;
        }
        writeln!(f, "    _ => ({}, {}),", self.defaults.0.saturating_sub(1), self.defaults.1.saturating_sub(1))?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl MaskBuilder {
    pub fn new(x: u32, y: u32) -> MaskBuilder {
        Self { map: BTreeMap::default(), defaults: (x, y) }
    }
    pub fn masks(&self) -> Vec<u8> {
        self.map.keys().copied().collect()
    }
    pub fn has_bits(&mut self, (x, y): (u32, u32), mask: &[u32]) {
        let s: u32 = mask.iter().map(|i| 2u32.pow(*i)).sum();
        self.has_mask((x, y), s as u8);
    }
    pub fn has_mask(&mut self, (x, y): (u32, u32), mask: u8) {
        let pop = self.map.insert(mask, (x, y));
        if let Some((i, j)) = pop {
            panic!("duplicate mask {}: new {:?}, old {:?}", mask, (x, y), (i, j))
        }
    }
    pub fn complete_set() -> Self {
        let mut masks = MaskBuilder::new(1, 4);
        // part1
        masks.has_bits((1, 1), &[4]);
        masks.has_bits((2, 1), &[2, 4]);
        masks.has_bits((3, 1), &[2, 4, 6]);
        masks.has_bits((4, 1), &[4, 6]);
        masks.has_bits((1, 2), &[0, 4]);
        masks.has_bits((2, 2), &[0, 2, 4]);
        masks.has_bits((3, 2), &[0, 2, 4, 6]);
        masks.has_bits((4, 2), &[0, 4, 6]);
        masks.has_bits((1, 3), &[0]);
        masks.has_bits((2, 3), &[0, 2]);
        masks.has_bits((3, 3), &[0, 2, 6]);
        masks.has_bits((4, 3), &[0, 6]);
        // <default>
        masks.has_bits((2, 4), &[2]);
        masks.has_bits((3, 4), &[2, 6]);
        masks.has_bits((4, 4), &[6]);
        // part2, &[0, 1, 2, 3, 4, 5, 6, 7]
        masks.has_bits((5, 1), &[0, 2, 4, 6, 7]);
        masks.has_bits((6, 1), &[2, 3, 4, 6]);
        masks.has_bits((7, 1), &[2, 4, 5, 6]);
        masks.has_bits((8, 1), &[0, 1, 2, 4, 6]);
        masks.has_bits((5, 2), &[0, 2, 3, 4]);
        masks.has_bits((6, 2), &[0, 1, 2, 3, 4, 5, 6]);
        masks.has_bits((7, 2), &[0, 2, 3, 4, 5, 6, 7]);
        masks.has_bits((8, 2), &[0, 4, 5, 6]);
        masks.has_bits((5, 3), &[0, 1, 2, 4]);
        masks.has_bits((6, 3), &[0, 1, 2, 3, 4, 6, 7]);
        masks.has_bits((7, 3), &[0, 1, 2, 4, 5, 6, 7]);
        masks.has_bits((8, 3), &[0, 4, 6, 7]);
        masks.has_bits((5, 4), &[0, 2, 4, 5, 6]);
        masks.has_bits((6, 4), &[0, 1, 2, 6]);
        masks.has_bits((7, 4), &[0, 2, 6, 7]);
        masks.has_bits((8, 4), &[0, 2, 3, 4, 6]);
        // part3
        masks.has_bits((9, 1), &[2, 3, 4]);
        masks.has_bits((10, 1), &[0, 2, 3, 4, 5, 6]);
        masks.has_bits((11, 1), &[2, 3, 4, 5, 6]);
        masks.has_bits((12, 1), &[4, 5, 6]);
        masks.has_bits((9, 2), &[0, 1, 2, 3, 4]);
        masks.has_bits((10, 2), &[0, 1, 2, 4, 5, 6]);
        masks.has_bits((11, 2), &[]);
        masks.has_bits((12, 2), &[0, 2, 4, 5, 6, 7]);
        masks.has_bits((9, 3), &[0, 1, 2, 3, 4, 6]);
        masks.has_bits((10, 3), &[0, 1, 2, 3, 4, 5, 6, 7]);
        masks.has_bits((11, 3), &[0, 2, 3, 4, 6, 7]);
        masks.has_bits((12, 3), &[0, 4, 5, 6, 7]);
        masks.has_bits((9, 4), &[0, 1, 2]);
        masks.has_bits((10, 4), &[0, 1, 2, 6, 7]);
        masks.has_bits((11, 4), &[0, 1, 2, 4, 6, 7]);
        masks.has_bits((12, 4), &[0, 6, 7]);

        masks
    }
    pub fn blob7x7_set() -> Self {
        let mut masks = MaskBuilder::new(1, 1);
        // part1
        // <default>
        masks.has_mask((2, 1), 4);
        masks.has_mask((3, 1), 92);
        masks.has_mask((4, 1), 124);
        masks.has_mask((5, 1), 116);
        masks.has_mask((6, 1), 80);
        // <excess>
        masks.has_mask((1, 2), 16);
        masks.has_mask((2, 2), 20);
        masks.has_mask((3, 2), 87);
        masks.has_mask((4, 2), 223);
        masks.has_mask((5, 2), 241);
        masks.has_mask((6, 2), 21);
        masks.has_mask((7, 2), 64);

        masks.has_mask((1, 3), 29);
        masks.has_mask((2, 3), 117);
        masks.has_mask((3, 3), 85);
        masks.has_mask((4, 3), 71);
        masks.has_mask((5, 3), 221);
        masks.has_mask((6, 3), 125);
        masks.has_mask((7, 3), 112);

        masks.has_mask((1, 4), 31);
        masks.has_mask((2, 4), 253);
        masks.has_mask((3, 4), 113);
        masks.has_mask((4, 4), 28);
        masks.has_mask((5, 4), 127);
        masks.has_mask((6, 4), 247);
        masks.has_mask((7, 4), 209);

        masks.has_mask((1, 5), 23);
        masks.has_mask((2, 5), 199);
        masks.has_mask((3, 5), 213);
        masks.has_mask((4, 5), 95);
        masks.has_mask((5, 5), 255);
        masks.has_mask((6, 5), 245);
        masks.has_mask((7, 5), 81);

        masks.has_mask((1, 6), 5);
        masks.has_mask((2, 6), 84);
        masks.has_mask((3, 6), 93);
        masks.has_mask((4, 6), 119);
        masks.has_mask((5, 6), 215);
        masks.has_mask((6, 6), 193);
        masks.has_mask((7, 6), 17);

        masks.has_mask((1, 7), 0);
        masks.has_mask((2, 7), 1);
        masks.has_mask((3, 7), 7);
        masks.has_mask((4, 7), 197);
        masks.has_mask((5, 7), 69);
        masks.has_mask((6, 7), 68);
        masks.has_mask((7, 7), 65);

        masks
    }
}

pub fn convert_blob7x7a<P>(image: P) -> ImageResult<()>
where
    P: AsRef<Path>,
{
    let path = image.as_ref().canonicalize()?;
    let new_name = path.file_stem().and_then(|s| s.to_str()).map(|s| format!("{}-std.png", s)).unwrap();
    let raw = image::open(image.as_ref())?.to_rgba8();
    let new = GridCompleteAtlas::from_blob7x7a(&raw, raw.width() / 7, raw.height() / 7);
    new.save(path.with_file_name(new_name))
}

pub fn convert_edge4x4<P>(image: P) -> ImageResult<()>
where
    P: AsRef<Path>,
{
    let (raw, output) = image_with_new_path(image)?;
    let new = GridCompleteAtlas::from_edge4x4(&raw, raw.width() / 4, raw.height() / 4);
    new.save(output)
}

/// Convert a 4x6 grid of tiles into a 4x6 grid of tiles with the RPG Maker VX style
pub fn convert_rpg4x6<P>(image: P) -> ImageResult<()>
where
    P: AsRef<Path>,
{
    let (raw, output) = image_with_new_path(image)?;
    let rpg = GridCornerRMVX::new(&raw, (0, 0), (raw.width() / 4, raw.height() / 6))?;
    rpg.as_complete().save(output)
}

fn image_with_new_path<P>(image: P) -> ImageResult<(RgbaImage, PathBuf)>
where
    P: AsRef<Path>,
{
    let path = image.as_ref().canonicalize()?;
    let raw = image::open(image.as_ref())?.to_rgba8();
    let new_name = path.file_stem().and_then(|s| s.to_str()).map(|s| format!("{}-std.png", s)).unwrap();
    let new_path = path.with_file_name(new_name);
    Ok((raw, new_path))
}

pub(crate) fn save_as_png<P>(image: &RgbaImage, path: P) -> ImageResult<()>
where
    P: AsRef<Path>,
{
    image::save_buffer_with_format(path, &image, image.width(), image.height(), ColorType::Rgba8, ImageFormat::Png)
}
