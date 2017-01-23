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
