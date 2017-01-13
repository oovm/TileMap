use image::{GenericImageView, ImageResult, RgbaImage};
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

pub struct AnimationSlice {}
