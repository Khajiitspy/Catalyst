use image::{DynamicImage, ImageOutputFormat};
use rayon::prelude::*;
use std::fs;

const IMAGE_SIZES: &[u32] = &[50, 100, 200, 400, 800, 1200];
const IMAGE_DIR: &str = "./images";

pub fn save_image_variants(
    bytes: &[u8],
    base_name: &str,
) -> Result<(), image::ImageError> {
    fs::create_dir_all(IMAGE_DIR).ok();

    let image = image::load_from_memory(bytes)?;

    IMAGE_SIZES.par_iter().try_for_each(|size| {
        save_resized(&image, *size, base_name)
    })?;

    Ok(())
}

fn save_resized(
    image: &DynamicImage,
    size: u32,
    base_name: &str,
) -> Result<(), image::ImageError> {
    let resized = image.resize(size, size, image::imageops::FilterType::Lanczos3);

    let filename = format!("{}/{}_{}", IMAGE_DIR, size, base_name);
    let mut file = std::fs::File::create(&filename)?;

    resized.write_to(&mut file, ImageOutputFormat::WebP)?;

    Ok(())
}
