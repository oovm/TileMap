use super::*;
use std::path::PathBuf;

/// Combining multiple sequence frame sprites into one animation frame sprites
///
/// # Arguments
///
/// * `folder`:
/// * `names`:
/// * `target`:
///
/// returns: Result<AnimationFrame, ImageError>
///
/// # Examples
///
/// ```
/// # use tileset::AnimationFrame;
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnimationFrame {
    /// describe how to get the sprites, the name in memory, the relative path name in disk
    key: String,
    /// The width of the cell in pixels
    cell_w: u32,
    /// The height of the cell in pixels
    cell_h: u32,
    /// The number of sprites
    frames: usize,
}

impl AnimationFrame {
    /// Combining multiple sequence frame sprites into one animation frame sprites
    ///
    /// # Arguments
    ///
    /// * `folder`:
    /// * `names`:
    /// * `target`:
    ///
    /// returns: Result<AnimationFrame, ImageError>
    ///
    /// # Examples
    ///
    /// ```
    /// # use tileset::AnimationFrame;
    /// ```
    pub fn composite_sequence_frames<P, S>(folder: P, names: &[S], target: &str) -> ImageResult<AnimationFrame>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let folder = folder.as_ref().canonicalize()?;
        let first = names.first().expect("The names slice must not be empty").as_ref();
        let (cell_w, cell_h) = image::open(folder.join(first))?.dimensions();
        let mut output = RgbaImage::new(cell_w, cell_h * names.len() as u32);
        for (i, file) in names.iter().enumerate() {
            let image = image::open(folder.join(file.as_ref()))?.to_rgba8();
            let y = i as i64 * cell_h as i64;
            image::imageops::overlay(&mut output, &image, 0, y);
        }
        let file_name = format!("{}.png", target);
        output.save(folder.join(&file_name))?;
        Ok(AnimationFrame { cell_w, cell_h, frames: names.len(), key: file_name })
    }
}

impl AnimationFrame {
    /// Combining multiple sequence frame sprites into one animation frame sprites
    ///
    /// # Examples
    ///
    /// ```
    /// # use tileset::AnimationFrame;
    /// ```
    pub fn get_key(&self) -> &str {
        &self.key
    }
    /// Get the image path if it is a disk image
    ///
    /// # Examples
    ///
    /// ```
    /// # use tileset::AnimationFrame;
    /// ```
    pub fn get_path(&self, root: &Path) -> PathBuf {
        root.join(&self.key)
    }
}
