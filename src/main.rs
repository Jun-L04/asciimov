use clap::Parser;
//use std::{fmt::format, io::Write};

mod image_processing;
mod ascii_fy;
use image_processing::{get_scale_factor, grayscale, scale_image};
use ascii_fy::{convert_to_ascii, print_to_console};
use std::path::Path;

// TODO I just want to grayscale an image
// TODO arguments for build-in parsing & conversion
    // maybe matching keywords from an array
// TODO possibly resizing terminal window?
// TODO pakcage into command line tool
// TODO add tests
// TODO colored ascii?
// TODO add logging
// TODO refine gitignore
// TODO have each function return the Result<something, error> thingy

// enum PathOption {
//     Pathbuf(PathBuf), 
//     String(String),
// }
#[derive(Parser)]
#[command(about = "A CLI tool for simple image processing.")]
struct Cli {
    // type of operation
    #[arg(help = "'as' for ascii-fying the image, 'gs' for grayscaling.")]
    operation: String,

    // optional, path parameter
    #[arg(help = "Path to the image to process.")]
    path: String //PathBuf,
    // resizing of terminal: true or false
}


fn main() {
    let args = Cli::parse();
    let operation = args.operation.as_str();
    let usr_path  = Path::new(&args.path);

    match operation {
        "as" => {
            ascii_sequence(&usr_path);
        }
        "gs" => {
            grayscale_sequence(&usr_path);
        }
        _ => {
            // unkown operation
            eprintln!("Unknown Operation: '{}'. \nTry '--help'.", operation);
        }
    }

}


fn ascii_sequence(usr_path: &Path) {
    let path = usr_path; //Path::new("src/img/blackhole.jpeg");
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


fn grayscale_sequence(usr_path: &Path) {
    let path = usr_path; //Path::new("src/img/blackhole.jpeg");
    // conversion to grayscale
    let grayscale_path = grayscale(path);
    println!("Done and saved to {}.", grayscale_path.display());
}
