mod image_filter;
mod float_image;

use image::{io::Reader as ImageReader, ImageBuffer, RgbImage};
use image_filter::{FilterMatrix};

use crate::float_image::{FImage, PixelFormat, Pixel};

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
    let mut fimage = FImage::new(input.width() as usize, input.height() as usize, PixelFormat::RGB);
    fimage.copy_from_image_buffer(&input);

    println!("Blurring...");
    let blurred = image_filter::filter_image(&fimage, FilterMatrix::new(GAUSSIAN));

    println!("Filtering...");
    let h = image_filter::filter_image(&blurred, FilterMatrix::new(GRADIENT_H));
    let v = image_filter::filter_image(&blurred, FilterMatrix::new(GRADIENT_V));

    // square and invert
    let h_sqinv = image_filter::fn_filter(&h, |x, y, p| {
        let t = Pixel::rgba(-1.0 * (p.r() * p.r()), -1.0 * (p.g() * p.g()), -1.0 * (p.b() * p.b()), p.a());
        let v = t.slice()[..p.slice().len()].to_owned();

        Pixel::from_boxed_slice(v.into_boxed_slice())
    });
    let v_sqinv = image_filter::fn_filter(&v, |x, y, p| {
        let t = Pixel::rgba(-1.0 * (p.r() * p.r()), -1.0 * (p.g() * p.g()), -1.0 * (p.b() * p.b()), p.a());
        let v = t.slice()[..p.slice().len()].to_owned();

        Pixel::from_boxed_slice(v.into_boxed_slice())
    });

    let combined = image_filter::combine_images(&h_sqinv, &v_sqinv);
    let mono = image_filter::combine_color_channels(&combined);

    let mut output = RgbImage::new(input.width(), input.height());
    mono.copy_to_image_buffer(&mut output);

    println!("Done.");
    output.save("output.png").unwrap();
}
