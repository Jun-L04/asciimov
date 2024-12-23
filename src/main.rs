use clap::Parser;
use image::{RgbaImage, imageops::FilterType};
use std::fs::OpenOptions;
use std::io::Write;
use std::{thread, time};
use indicatif::ProgressBar;



// TODO file path based on windows/unix-like system
// TODO ascii conversion
// TODO resizing image and possibly resizing terminal window?
// TODO pakcage into command line tool
// TODO add tests
// TODO colored ascii?
// TODO add logging
/// Search for a pattern in a file and display the lines that contain it.
/// 
#[derive(Parser)]
struct Cli {
    // name of the character we want in ascii, it is just bocchi for now
    character_name: String,
    // something else maybe? another arugment?
    // path: std::path::PathBuf,
}

// fn main() {
//     let args = Cli::parse();

//     let content = std::fs::read_to_string(&args.path).expect("could not read file");

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }

//     println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
// }

fn main() {
    let path = "src\\img\\bocchi.jpeg";
    let grayscale_path = grayscale(path);
    
    let scaled_path = scale_image(grayscale_path, 100, 100);

    convert_to_ascii(scaled_path);

    print_to_console().expect("Unable to Print to Console");
}

fn grayscale(path: &str) -> &str {
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
    let output_path = "src\\img\\grayscale.png";
    img.save(output_path)
        .expect("Unable to Save Grayscaled Image");

    return &output_path
}


fn scale_image(input_path: &str, new_width: u32, new_height: u32) -> &str {
    let output_path = "src\\img\\scaled.png";
    // Open the image file
    let img = image::open(input_path).expect("Failed to open image");

    // Resize the image using the Lanczos3 filter
    let resized_img = img.resize(new_width, new_height, FilterType::Lanczos3);

    // Save the resized image to the specified path
    resized_img.save(output_path).expect("Failed to save image");
    return output_path;
}


fn convert_to_ascii(graysacled_path: &str) {
    let ascii_file_path = "ascii.txt";
    let mut ascii_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(ascii_file_path)
        .expect("Unable to Open File at {ascii_file_path}");

    let (ascii_vec, length) = get_ascii();
    // open the grayscale image
    let img: RgbaImage = image::open(graysacled_path)
        .expect("Cannot Open Grayscale Image in Path: {path}")
        .to_rgba8();

    let (width, height): (u32, u32) = img.dimensions();

    // row-wise opertaions
    let pb = indicatif::ProgressBar::new((width * height) as u64);
    for y in 0..height {
        let mut line_of_ascii: Vec<char> = Vec::new();
        for x in 0..width {
            // retreive each pixel
            let pixel = img.get_pixel(x, y);
            let index = ((pixel[0] as f32 / 255.0) * (length as f32 - 1.0)) as usize;
            let aschii_char = ascii_vec[index];
            line_of_ascii.push(aschii_char);
        }
        writeln!(ascii_file, "{}", line_of_ascii.iter().collect::<String>())
            .expect("Failed to Write ASCII Line to File");
        pb.inc(1);
    }
}

fn get_ascii() -> (Vec<char>, u8) {
    let all_ascii = ".:;-=+*]}!|#$%&@".to_string();

    let ascii_vec: Vec<char> = all_ascii.chars().collect();
    let length = ascii_vec.len() as u8;

    return (ascii_vec, length);
}


fn print_to_console() -> std::io::Result<()> {
    let stdout = std::io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it
    

    let ascii_file_path = "ascii.txt";
    let content = std::fs::read_to_string(ascii_file_path).expect("could not read file");
    
    for line in content.lines() {
        writeln!(handle, "{}", line)?; // add `?` if you care about errors here
    }

    Ok(())
}
