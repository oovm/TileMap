use image::ImageResult;
use std::path::Path;

pub fn decompose_grid_sequence_frame<P>(
    path: P,
    start: (usize, usize),
    end: (usize, usize),
    is_horizontal: bool,
) -> ImageResult<()>
where
    P: AsRef<Path>,
{
    todo!()
}

pub fn composite_sequence_frame<P>(folder: P, names: &[&str], target: &str) -> ImageResult<()>
where
    P: AsRef<Path>,
{
    assert_ne!(names.len(), 0, "The names slice must not be empty");
    let folder = folder.as_ref().canonicalize()?;
    let mut images = Vec::with_capacity(names.len());
    for name in names {
        let path = folder.join(name);
        let image = image::open(&path)?.to_rgba8();
        images.push(image);
    }
    let head = images.remove(0);
    let (cell_w, cell_h) = head.dimensions();
    let mut output = image::RgbaImage::new(cell_w, cell_h * images.len() as u32);
    for (i, image) in images.iter().enumerate() {
        let y = i as i64 * cell_h as i64;
        image::imageops::overlay(&mut output, image, 0, y);
    }
    output.save(folder.join(target))?;
    Ok(())
}
