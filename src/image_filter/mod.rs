mod filter_matrix;
pub use filter_matrix::FilterMatrix;
use image::{ImageBuffer, Pixel, GenericImage, Primitive};

fn get_pixel<SP: Primitive, P: Pixel<Subpixel = SP>, I: GenericImage<Pixel = P>>(x: i32, y: i32, img: &I) -> Option<P> {
    if x < 0 || y < 0 || x >= img.width() as i32 || y >= img.height() as i32 {
        return None;
    }
    
    Some(img.get_pixel(x as u32, y as u32))
}

fn filter_pixel<SP: Primitive, P: Pixel<Subpixel = SP>, I: GenericImage<Pixel = P>>(pixel: (P, u32, u32), img: &I, filter: &FilterMatrix) -> P {
    let a = { let mut t = SP::DEFAULT_MIN_VALUE; pixel.0.map_with_alpha(|sp| sp, |sp| {t = sp; sp}); t};
    
    let mut zero = pixel.0.clone();
    zero.apply(|_| SP::DEFAULT_MIN_VALUE);

    let mut sum = zero.clone();

    let range = filter.size()  as i32 / 2;
    for x in -range..=range {
        for y in -range..=range {
            let px = match get_pixel(pixel.1 as i32 + x, pixel.2 as i32 + y, img) {
                Some(s) => s,
                None => zero.clone()
            };

            sum.apply2(&px, |sp, o| {
                let t = o.to_u8().unwrap() as f32 * filter.get((x + range) as usize, (y + range) as usize);
                
                sp + SP::from(t as u32).unwrap()
            });
        }
    }


    sum.apply_with_alpha(|sp| {
        sp
    }, |_| a);
    sum
}

pub fn filter_image<P: Pixel, I: GenericImage<Pixel = P>>(img: &I, filter: FilterMatrix) -> ImageBuffer<P, Vec<<P as Pixel>::Subpixel>> {
    let mut out = ImageBuffer::new(img.width(), img.height());
    
    for x in 0..img.width() {
        for y in 0..img.height() {
            let px = img.get_pixel(x, y);

            out.put_pixel(x, y, filter_pixel((px, x, y), img, &filter));
        }
    }

    out
}