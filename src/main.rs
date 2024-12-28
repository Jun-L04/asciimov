use clap::Parser;
use std::io::Write;

mod image_processing;
mod ascii_fy;
use image_processing::{get_scale_factor, grayscale, scale_image};
use ascii_fy::convert_to_ascii;

// TODO I just want to grayscale an image
// TODO arguments for build-in parsing & conversion
    // maybe matching keywords from an array
// TODO file path based on windows/unix-like system
// TODO possibly resizing terminal window?
// TODO pakcage into command line tool
// TODO add tests
// TODO colored ascii?
// TODO add logging
// TODO refine gitignore
// TODO have each function return the Result<something, error> thingy

/// Search for a pattern in a file and display the lines that contain it.
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
    let path = "src\\img\\blackhole.jpeg";
    // conversion to grayscale
    let grayscale_path = grayscale(path);
    // new dimensions for scaling
    let (new_width, new_height) = get_scale_factor(grayscale_path).expect("unexpected get_scale_factor() failure.");
    // scales grayscaled image
    let scaled_path = scale_image(grayscale_path, new_width, new_height);
    // converting to ascii
    convert_to_ascii(scaled_path);
    // printing resutls
    print_to_console().expect("unexpected print_to_console() failure.");
}


fn print_to_console() -> std::io::Result<()> {
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
