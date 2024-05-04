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

    optimize_images()
}

fn optimize_images() {
    // Optimize images to webp
    let run_image_optimization = env::var(RUN_IMAGE_OPTIMIZATION_ENV_VAR)
        .unwrap_or(String::from("false"))
        .parse::<bool>()
        .unwrap_or(false);

    if !run_image_optimization {
        return;
    }

    let Ok(entries) = fs::read_dir(RAW_IMAGES_DIRECTORY) else {
        return;
    };

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();
        let (Some(filename), Some(filestem)) = (path.file_name(), path.file_stem()) else {
            continue;
        };
        if let Ok(img) = image::open(entry.path()) {
            let output_path = Path::new(OPTIMIZED_IMAGES_DIRECTORY)
                .join(filestem)
                .with_extension("webp");
            if output_path.exists() {
                continue;
            }
            let (w, h) = img.dimensions();

            let min = std::cmp::min(w, h) as f64;
            let size_factor = 256.0 / min;
            let img: DynamicImage = image::DynamicImage::ImageRgba8(imageops::resize(
                &img,
                (w as f64 * size_factor) as u32,
                (h as f64 * size_factor) as u32,
                imageops::FilterType::Triangle,
            ));

            let encoder: Encoder = Encoder::from_image(&img).unwrap();
            // Encode the image at a lossy quality of 90
            let webp: WebPMemory = encoder.encode(90f32);
            std::fs::write(&output_path, &*webp).unwrap();
        } else {
            let output_path = Path::new(OPTIMIZED_IMAGES_DIRECTORY).join(filename);
            if output_path.exists() {
                continue;
            }
            let _ = std::fs::copy(entry.path(), output_path);
        }
    }
}
