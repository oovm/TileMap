use image::{GenericImageView, ImageResult};
use std::path::Path;

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
            let mut out = dir.join(format!("{}-{}-{}.png", name.to_str().unwrap(), col, row));
            view.to_image().save(&out)?;
        }
    }
    Ok(())
}

pub struct AnimationSlice {}

pub fn grid_corner_mask(lu: bool, ru: bool, ld: bool, rd: bool) -> u8 {
    (lu as u8) << 0 | (ru as u8) << 1 | (ld as u8) << 2 | (rd as u8) << 3
}
