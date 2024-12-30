use image::{RgbaImage, imageops::FilterType};
use crossterm::terminal;
use std::path::Path;
use std::env;


pub fn grayscale(path: &Path) -> &Path {
    let mut img: RgbaImage = image::open(path)
        .expect("Cannot Open Image in Path: {path}")
        .to_rgba8();

    let (width, height): (u32, u32) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            // retreive each pixel
            let pixel = img.get_pixel_mut(x, y);

            let avg = (pixel.0[0..3]
                .iter()
                .copied()
                .fold(0.0, |acc, x| acc + x as f32)
                / 3.0)
                .round() as u8;

            pixel[0] = avg; // R
            pixel[1] = avg; // G
            pixel[2] = avg; // B
        }
    }
    let output_path = Path::new("src/img/grayscale.png");
    img.save(output_path)
        .expect("Unable to Save Grayscaled Image");

    return &output_path
}


pub fn scale_image(input_path: &Path, new_width: u32, new_height: u32) -> &Path {
    let output_path = Path::new("src/img/scaled.png");
    // open image
    let img = image::open(input_path).expect("Failed to Open Image");

    // image resizing using the Lanczos3 filter
    let resized_img = img.resize(new_width, new_height, FilterType::Lanczos3);

    // saving scaled image to the specified path
    resized_img.save(output_path).expect("Failed to Save Scaled Image");
    return &output_path;
}


pub fn get_scale_factor(output_path: &Path) -> Result<(u32, u32), String> {

    let img: RgbaImage = image::open(output_path)
        .expect("Cannot Open Image in Path: {path}")
        .to_rgba8();

    let (img_width, img_height): (u32, u32) = img.dimensions();

    match terminal::size() {
        Ok((term_width, term_height)) => {
            println!("Terminal size is: {} by {}", term_width, term_height);
            let (term_width, term_height) = (term_width as u32, term_height as u32);
            if  term_width < 30 && term_height < 30{ // terminal too small
                return Err("Terminal Too Small!".to_string());
            } else if term_width >= img_width && term_height >= img_height { // image fits
                return Ok((img_width, img_height));
            } else { // scaling
                // always tries to fit into the terminal size
                let width_scale = term_width as f32 / img_width as f32;
                let height_scale = term_height as f32 / img_height as f32;

                let scale = width_scale.min(height_scale);

                let new_width = (scale * img_width as f32).round() as u32;
                let new_height = (scale * img_height as f32).round() as u32;

                Ok((new_width, new_height))
            }
        }
        Err(e) => Err(format!("Failed to Get Terminal Dimensions. {}", e)),
    }
}


// pub fn clean_up_image() {
//     // scaling image is an intermediate step, we delete it afterwards
// }