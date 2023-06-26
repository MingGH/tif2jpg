extern crate image;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: tif2jpg <input_file> <output_file>");
        return;
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    let img = image::open(input_path).unwrap();
    let _ = img.save(output_path).unwrap();
}