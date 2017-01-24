use image::ImageResult;
use std::path::Path;

pub fn decompose_grid_sequence_frame<P>(
    _path: P,
    _start: (usize, usize),
    _end: (usize, usize),
    _is_horizontal: bool,
) -> ImageResult<()>
where
    P: AsRef<Path>,
{
    todo!()
}

pub struct AnimationSlice {}

pub fn grid_corner_mask(lu: bool, ru: bool, ld: bool, rd: bool) -> u8 {
    (lu as u8) << 0 | (ru as u8) << 1 | (ld as u8) << 2 | (rd as u8) << 3
}
