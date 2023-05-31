mod filter_matrix;
pub use filter_matrix::FilterMatrix;
use image::{ImageBuffer, Pixel, GenericImage, Primitive, Rgba};

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

    let mut sum = vec![0.0f32; P::CHANNEL_COUNT as usize];

    let range = filter.size()  as i32 / 2;
    for x in -range..=range {
        for y in -range..=range {
            let px = match get_pixel(pixel.1 as i32 + x, pixel.2 as i32 + y, img) {
                Some(s) => s,
                None => zero.clone()
            };

            for (i, sp) in px.channels().iter().enumerate() {
                let t = sp.to_u8().unwrap() as f32 * filter.get((x + range) as usize, (y + range) as usize);
            
                sum[i] += t;
            }
        }
    }

    let mut out = zero.clone();
    for (i, sp) in out.channels_mut().iter_mut().enumerate() {
        *sp = SP::from((sum[i] as i32).abs().clamp(0, 255) as u8).unwrap();
    }
    out.apply_with_alpha(|sp| {
        sp
    }, |_| a);
    out
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

pub fn combine_images<SP: Primitive, P: Pixel<Subpixel = SP>, I: GenericImage<Pixel = P>>(img1: &I, img2: &I) -> ImageBuffer<P, Vec<<P as Pixel>::Subpixel>> {
    let mut out = ImageBuffer::new(img1.width(), img1.height());
    
    for x in 0..img1.width() {
        for y in 0..img1.height() {
            let px1 = img1.get_pixel(x, y);
            let px2 = img2.get_pixel(x, y);

            let mut px_out = px1.clone();
            px_out.apply2(&px2, |sp, o| {
                SP::from((sp.to_f32().unwrap() + o.to_f32().unwrap()) / 2.0).unwrap()
            });

            out.put_pixel(x, y, px_out);
        }
    }

    out
}

pub fn combine_color_channels<SP: Primitive, P: Pixel<Subpixel = SP>, I: GenericImage<Pixel = P>>(img: &I) -> ImageBuffer<P, Vec<<P as Pixel>::Subpixel>> {
    let mut out = ImageBuffer::new(img.width(), img.height());
    
    for x in 0..img.width() {
        for y in 0..img.height() {
            let mut px = img.get_pixel(x, y);

            let mut sum = 0;
            let mut count = 0;

            px.apply_without_alpha(|sp| {
                sum += sp.to_u8().unwrap() as u32;
                count += 1;

                sp
            });

            px.apply_without_alpha(|_| {
                SP::from(sum / count).unwrap()
            });

            out.put_pixel(x, y, px);
        }
    }

    out
}