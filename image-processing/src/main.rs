use glob::glob;
use std::path::PathBuf;
use image::ImageReader;

fn main() {
    let mut last_image_opt: Option<image::RgbImage> = None;
    let mut last_path_opt: Option<PathBuf> = None;
    let mut index = 0;

    for entry in glob("./FrameDataset/01/*.png").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("Processing {:?}", path.display());
                index += 1;

                let img = ImageReader::open(path.clone()).unwrap().decode().unwrap().to_rgb8();
                if let (Some(mut last_image), Some(last_path)) = (last_image_opt, last_path_opt) {
                    // Modify last image and save as new image with diffs.
                    last_image.pixels_mut().into_iter().zip(img.pixels()).for_each(|(p1, p2)| {
                        let total_difference: i64 = (p1[0] as i64 - p2[0] as i64).abs() +
                            (p1[1] as i64 - p2[1] as i64).abs() +
                            (p1[2] as i64 - p2[2] as i64).abs();

                        let max_difference: i64 = 255 * 3;
                        let min_difference: i64 = 0;

                        let grayscaled_difference = (255 * total_difference / (max_difference - min_difference)) as u8;

                        p1[0] = grayscaled_difference;
                        p1[1] = grayscaled_difference;
                        p1[2] = grayscaled_difference;
                    });

                    last_image.save(format!("./ProcessedDifference/01/{index:07}.png")).unwrap();
                }

                last_path_opt = Some(path.clone());
                last_image_opt = Some(img);
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
