mod image_filter;
use image::io::Reader as ImageReader;
use image_filter::{filter_image, FilterMatrix};

const MATRIX: [[f32; 3]; 3] = [[0.0, 0.0, 0.0],
                               [0.0, 1.0, 0.0],
                               [0.0, 0.0, 0.0]];

fn main() {
    let input = ImageReader::open("input.png").unwrap().decode().unwrap();

    let filter = FilterMatrix::new(MATRIX);
    println!("Filtering...");
    let output = filter_image(&input, filter);
    println!("Done.");
    output.save("output.png").unwrap();
}
