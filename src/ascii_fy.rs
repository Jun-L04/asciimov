use image::RgbaImage;
use std::fs::OpenOptions;
use std::io::Write;
use indicatif::ProgressBar;
use std::path::Path;

pub fn convert_to_ascii(graysacled_path: &Path) {
    let ascii_file_path = Path::new("ascii.txt");
    let mut ascii_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(ascii_file_path)
        .expect("Unable to Open File at {ascii_file_path}");

    let (ascii_vec, length) = get_ascii();
    // open the grayscale image
    let img: RgbaImage = image::open(graysacled_path)
        .expect(&format!("Cannot Open Grayscale Image in Path: {}", graysacled_path.display()))
        .to_rgba8();

    let (width, height): (u32, u32) = img.dimensions();

    // row-wise opertaions
    let pb = ProgressBar::new(height as u64);
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
    //pb.finish();
}

fn get_ascii() -> (Vec<char>, u8) {
    let all_ascii = ".:;-=+*]}!|#$%&@".to_string();

    let ascii_vec: Vec<char> = all_ascii.chars().collect();
    let length = ascii_vec.len() as u8;

    return (ascii_vec, length);
}


pub fn print_to_console() -> std::io::Result<()> {
    let stdout = std::io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it
    

    let ascii_file_path = "ascii.txt";
    let content = std::fs::read_to_string(ascii_file_path).expect("could not read file");
    // let pb = indificati::ProgressBar
    for line in content.lines() {
        writeln!(handle, "{}", line)?; // add `?` if you care about errors here
    }

    Ok(())
}