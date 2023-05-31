mod image_filter;
use image::io::Reader as ImageReader;
use image_filter::{filter_image, FilterMatrix};

const GRADIENT_H: [[f32; 3]; 3] = [[-1.0, 0.0, 1.0],
                               [-2.0, 0.0, 2.0],
                               [-1.0, 0.0, 1.0]];

const GRADIENT_V: [[f32; 3]; 3] = [[1.0, 2.0, 1.0],
                               [0.0, 0.0, 0.0],
                               [-1.0, -2.0, -1.0]];


const GAUSSIAN: [[f32; 9]; 9] =   [[0.0000, 0.0000, 0.0000, 0.0001, 0.0001, 0.0001, 0.0000, 0.0000, 0.0000],
                               [0.0000,	0.0000,	0.0004,	0.0014,	0.0023,	0.0014,	0.0004,	0.0000,	0.0000],
                               [0.0000,	0.0004,	0.0037,	0.0146,	0.0232,	0.0146,	0.0037,	0.0004,	0.0000],
                               [0.0001,	0.0014,	0.0146,	0.0584,	0.0926,	0.0584,	0.0146,	0.0014,	0.0001],
                               [0.0001,	0.0023,	0.0232,	0.0926,	0.1466,	0.0926,	0.0232,	0.0023,	0.0001],
                               [0.0001,	0.0014,	0.0146,	0.0584,	0.0926,	0.0584,	0.0146,	0.0014,	0.0001],
                               [0.0000,	0.0004,	0.0037,	0.0146,	0.0232,	0.0146,	0.0037,	0.0004,	0.0000],
                               [0.0000,	0.0000,	0.0004,	0.0014,	0.0023,	0.0014,	0.0004,	0.0000,	0.0000],
                               [0.0000,	0.0000,	0.0000,	0.0001,	0.0001,	0.0001,	0.0000,	0.0000,	0.0000]];
fn main() {
    let input = ImageReader::open("input.png").unwrap().decode().unwrap();

    let blurred = filter_image(&input, FilterMatrix::new(GAUSSIAN));

    println!("Filtering...");
    let h = filter_image(&blurred, FilterMatrix::new(GRADIENT_H));
    let v = filter_image(&blurred, FilterMatrix::new(GRADIENT_V));

    let combined = image_filter::combine_images(&h, &v);
    let mono = image_filter::combine_color_channels(&combined);

    let output = mono;
    println!("Done.");
    output.save("output.png").unwrap();
}
