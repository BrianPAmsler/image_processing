mod filter_matrix;
pub use filter_matrix::FilterMatrix;
use image::{ImageBuffer, GenericImage, Primitive, Rgba};

use crate::float_image::{Pixel, FImage, PixelFormat};

fn filter_pixel<'a>(pixel: (Pixel, i32, i32), img: &'a FImage, filter: &FilterMatrix) -> Pixel<'a> {
    let a = pixel.0.a();

    let mut sum = vec![0.0f32; pixel.0.format().channel_count()];
    if sum.len() == 4 {
        sum[3] = a;
    }

    let range = filter.size()  as i32 / 2;
    for x in -range..=range {
        for y in -range..=range {
            let px = img.get_pixel(pixel.1 + x, pixel.2 + y);

            for (i, sp) in px.slice().iter().enumerate() {
                let t = *sp * filter.get((x + range) as usize, (y + range) as usize);
            
                sum[i] += t;
            }
        }
    }

    Pixel::from_boxed_slice(sum.into_boxed_slice())
}

pub fn filter_image(img: &FImage, filter: FilterMatrix) -> FImage {
    let mut out = FImage::new(img.width(), img.height(), img.get_pixel_format());
    
    for x in 0..img.width() {
        for y in 0..img.height() {
            let px = img.get_pixel(x as i32, y as i32);

            out.set_pixel(x as i32, y as i32, filter_pixel((px, x as i32, y as i32), img, &filter));
        }
    }

    out
}

pub fn combine_images(img1: &FImage, img2: &FImage) -> FImage {
    let mut out = FImage::new(img1.width(), img1.height(), img1.get_pixel_format());
    
    for x in 0..img1.width() {
        for y in 0..img1.height() {
            let mut px1 = img1.get_pixel(x as i32, y as i32).slice().to_owned();
            let px2 = img2.get_pixel(x as i32, y as i32);

            for (i, p) in px1.iter_mut().enumerate() {
                *p += px2.slice()[i];
            }

            out.set_pixel(x as i32, y as i32, Pixel::from_slice(&px1));
        }
    }

    out
}

pub fn combine_color_channels(img: &FImage) -> FImage {
    let mut out = FImage::new(img.width(), img.height(), PixelFormat::Mono);
    
    for x in 0..img.width() {
        for y in 0..img.height() {
            let px = img.get_pixel(x as i32, y as i32);

            let sum = px.r() + px.g() + px.b();

            out.set_pixel(x as i32, y as i32, Pixel::mono(sum));
        }
    }

    out
}