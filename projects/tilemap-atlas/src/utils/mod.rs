use image::error::{LimitError, LimitErrorKind};
use image::{ImageError, ImageResult};


pub fn dimension_error<T>() -> ImageResult<T> {
    Err(ImageError::Limits(LimitError::from_kind(LimitErrorKind::DimensionError)))
}