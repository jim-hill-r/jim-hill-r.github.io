use image::*;
use std::path::Path;
use std::{env, fs};
use webp::*;

const RAW_IMAGES_DIRECTORY: &str = "content/images";
const OPTIMIZED_IMAGES_DIRECTORY: &str = "public/images";
const RUN_IMAGE_OPTIMIZATION_ENV_VAR: &str = "PF_RUN_IMAGE_OPTIMIZATION";
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed={}", RAW_IMAGES_DIRECTORY);

    // Optimize images to webp
    if let Ok(run_image_optimization) = env::var(RUN_IMAGE_OPTIMIZATION_ENV_VAR) {
        if run_image_optimization == "true" {
            if let Ok(entries) = fs::read_dir(RAW_IMAGES_DIRECTORY) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(img) = image::open(entry.path()) {
                            if let Some(filestem) = entry.path().file_stem() {
                                let output_path = Path::new(OPTIMIZED_IMAGES_DIRECTORY)
                                    .join(filestem)
                                    .with_extension("webp");
                                if !output_path.exists() {
                                    let (w, h) = img.dimensions();

                                    let min = std::cmp::min(w, h) as f64;
                                    let size_factor = 256.0 / min;
                                    let img: DynamicImage =
                                        image::DynamicImage::ImageRgba8(imageops::resize(
                                            &img,
                                            (w as f64 * size_factor) as u32,
                                            (h as f64 * size_factor) as u32,
                                            imageops::FilterType::Triangle,
                                        ));

                                    let encoder: Encoder = Encoder::from_image(&img).unwrap();
                                    // Encode the image at a lossy quality of 90
                                    let webp: WebPMemory = encoder.encode(90f32);
                                    let output_path = Path::new(OPTIMIZED_IMAGES_DIRECTORY)
                                        .join(filestem)
                                        .with_extension("webp");
                                    std::fs::write(&output_path, &*webp).unwrap();
                                }
                            }
                        } else if let Some(filename) = entry.path().file_name() {
                            let output_path = Path::new(OPTIMIZED_IMAGES_DIRECTORY).join(filename);
                            if !output_path.exists() {
                                let _ = std::fs::copy(entry.path(), output_path);
                            }
                        }
                    }
                }
            }
        }
    }
}
