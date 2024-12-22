use clap::Parser;
use image::RgbaImage;
use std::fs::OpenOptions;
use std::io::Write;

// TODO file path based on windows/unix-like system
// TODO ascii conversion
// TODO resizing image and possibly resizing terminal window?
// TODO pakcage into command line tool
// TODO colored ascii?
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // name of the character we want in ascii, it is just bocchi for now
    character_name: String,
    // something else maybe? another arugment?
    path: std::path::PathBuf,
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
    grayscale(path);

    let grayscale_path = "src\\img\\grayscale.png";
    convert_to_ascii(grayscale_path)
}

fn grayscale(path: &str) {
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
}

fn convert_to_ascii(graysacled_path: &str) {
    let ascii_file_path = "ascii.txt";
    let mut ascii_file = OpenOptions::new()
        .append(true)
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
    for y in 0..height {
        let mut line_of_ascii: Vec<char> = Vec::new();
        for x in 0..width {
            // retreive each pixel
            let pixel = img.get_pixel(x, y);
            let index = (pixel[0] / 255 * length) as usize;
            let aschii_char = ascii_vec[index];
            line_of_ascii.push(aschii_char);
        }
        writeln!(ascii_file, "{}", line_of_ascii.iter().collect::<String>())
            .expect("Failed to Write ASCII Line to File");
    }
}

fn get_ascii() -> (Vec<char>, u8) {
    let all_ascii = ".:;-=+*]}!|#$%&@".to_string();

    let ascii_vec: Vec<char> = all_ascii.chars().collect();
    let length = ascii_vec.len() as u8;

    return (ascii_vec, length);
}
