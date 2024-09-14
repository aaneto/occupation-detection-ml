use glob::glob;
use std::path::PathBuf;
use image::ImageReader;

const CENTER: (u32, u32) = (320, 180);
const RECT_SIZE: (u32, u32) = (30, 30);
const MOVEMENT_THRESHOLD: u8 = 50;


fn main() {
    let mut last_image_opt: Option<image::RgbImage> = None;
    let mut last_path_opt: Option<PathBuf> = None;
    let mut index = 0;
    let rect_points = rect_points();

    // Images are 640 x 360
    // Rect is at CENTER and is 50x200.
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
                    let mut is_movement_inside_rect: bool = false;

                    for &(x, y) in rect_points.iter() {
                        let pixel_mut = last_image.get_pixel_mut(x, y);
                        if pixel_mut[0] > MOVEMENT_THRESHOLD {
                            is_movement_inside_rect = true;
                        }
                    }

                    for &(x, y) in rect_points.iter() {
                        let pixel_mut = last_image.get_pixel_mut(x, y);
                        if is_movement_inside_rect {
                            pixel_mut[0] = 200;
                            pixel_mut[1] = 0;
                            pixel_mut[2] = 0;
                        } else {
                            pixel_mut[0] = 0;
                            pixel_mut[1] = 200;
                            pixel_mut[2] = 0;
                        }
                    }

                    last_image.save(format!("./ProcessedDifference/01/{index:07}.png")).unwrap();
                }

                last_path_opt = Some(path.clone());
                last_image_opt = Some(img);
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

fn rect_points() -> Vec<(u32, u32)> {
    let mut points = Vec::new();

    let x1 = CENTER.0 - RECT_SIZE.0;
    let x2 = CENTER.0 + RECT_SIZE.0;

    let y1 = CENTER.1 - RECT_SIZE.1;
    let y2 = CENTER.1 + RECT_SIZE.1;

    for x in x1..x2 {
        for y in y1..y2 {
            points.push((x, y));
        }
    }

    points
}
