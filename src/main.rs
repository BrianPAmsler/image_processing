mod image_filter;
mod float_image;
mod circle_drawer;
mod ishihara_generator;

use image::{io::Reader as ImageReader, ImageBuffer, RgbImage, RgbaImage};
use image_filter::{FilterMatrix};

use crate::{float_image::{FImage, PixelFormat, Pixel}, ishihara_generator::generate_circles, circle_drawer::{draw_circle, fill_circle}};

const bg_colors: [&'static str; 7] = ["#cf5f47", "#cf5f47", "#fd9500", "#ffd500", "#ee8568", "#ee8568", "#eebd7a"];

const fg_colors: [&'static str; 3] = ["#5a8a50", "#a2ab5a", "#c9cc7d"];

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
    let mut fimage = FImage::new(input.width() as usize, input.height() as usize, PixelFormat::RGBA);
    fimage.copy_from_image_buffer(&input);

    let mut ishihara_canvas = FImage::new(fimage.width(), fimage.height(), PixelFormat::RGBA);
    // println!("Blurring...");
    // let blurred = image_filter::filter_image(&fimage, FilterMatrix::new(GAUSSIAN));

    // println!("Filtering...");
    // let h = image_filter::filter_image(&blurred, FilterMatrix::new(GRADIENT_H));
    // let v = image_filter::filter_image(&blurred, FilterMatrix::new(GRADIENT_V));

    // // square and invert
    // let h_sqinv = image_filter::fn_filter(&h, |x, y, p| {
    //     let t = Pixel::rgba(-1.0 * (p.r() * p.r()), -1.0 * (p.g() * p.g()), -1.0 * (p.b() * p.b()), p.a());
    //     let v = t.slice()[..p.slice().len()].to_owned();

    //     Pixel::from_boxed_slice(v.into_boxed_slice())
    // });
    // let v_sqinv = image_filter::fn_filter(&v, |x, y, p| {
    //     let t = Pixel::rgba(-1.0 * (p.r() * p.r()), -1.0 * (p.g() * p.g()), -1.0 * (p.b() * p.b()), p.a());
    //     let v = t.slice()[..p.slice().len()].to_owned();

    //     Pixel::from_boxed_slice(v.into_boxed_slice())
    // });

    // let combined = image_filter::combine_images(&h_sqinv, &v_sqinv);
    // let mut mono = image_filter::combine_color_channels(&combined);
    // mono.normalize();

    // println!("Watershed...");
    // let watershed = image_filter::watershed(&mono, (mono.width() as i32 / 2, mono.height() as i32 / 2), (10, 10));

    let c_min = (((fimage.width().pow(2) + fimage.height().pow(2)) as f32).sqrt() * 0.005) as usize;
    let c_max = c_min * 2;
    println!("Generating Circles...");
    let circles = generate_circles(fimage.width(), fimage.height(), c_min, c_max, (c_min / 2).max(1), 0.75);


    let fgs: Box<[Pixel]> = fg_colors.iter().map(|c| Pixel::from_hex(c)).collect();
    let bgs: Box<[Pixel]> = bg_colors.iter().map(|c| Pixel::from_hex(c)).collect();
    println!("Drawing Circles...");
    for circle in &circles[..] {
        let cat_pixel = fimage.get_pixel(circle.x as i32, circle.y as i32);
        let color = if cat_pixel.a() > 0.5 {
            let r = rand::random::<usize>() % fgs.len();
            fgs[r].clone()
        } else {
            let r = rand::random::<usize>() % bgs.len();
            bgs[r].clone()
        };
        fill_circle(circle.x as i32, circle.y as i32, circle.radius as i32, color, false, &mut ishihara_canvas);
    }

    let mut output = RgbaImage::new(input.width(), input.height());
    ishihara_canvas.copy_to_image_buffer(&mut output);

    println!("Done.");
    output.save("output.png").unwrap();
}
