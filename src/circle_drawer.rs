use crate::float_image::{Pixel, FImage};

fn draw_points(x: i32, y: i32, i: i32, j: i32, color: Pixel, img: &mut FImage) {
        // '''Draws 8 points, one on each octant.'''
        let mut coords = Vec::new();
        for k in 0..4i32 {
            let mx = (-1i32).pow(k as u32 % 2);
            let my = (-1i32).pow(k as u32 / 2);
            let p = (i * mx, j * my);
            
            // #Square symmetry              
            coords.push(p);
            // #Diagonal symmetry
            coords.push((p.1, p.0));
        }

        for (i_, j_) in coords {
            img.set_pixel_blended(x + i_, y + j_, color.to_owned());
        }
}

pub fn draw_circle(x: i32, y: i32, radius: i32, color: Pixel, wrap: bool, img: &mut FImage) {
    let mut i = radius;
    let mut j = 0;
    let mut t = 0;

    while i >= j {
        j += 1;
        let root = ((radius*radius - j*j) as f32).sqrt();
        let d = (255.0 * (root.ceil() - root) + 0.5).floor() as u8;
        let d_ = !d;

        if d < t {
            i -= 1;
        }

        //Draw points
        let p1 = Pixel::rgba(color.r(), color.g(), color.b(), d_ as f32 / 255.0);
        let p2 = Pixel::rgba(color.r(), color.g(), color.b(), d as f32 / 255.0);

        draw_points(x, y, i, j, p1, img);
        draw_points(x, y, i - 1, j, p2, img);

        t = d;
    }

    // Fill in gaps on axes
    img.set_pixel(x + radius, y, color.clone());
    img.set_pixel(x - radius, y, color.clone());
    img.set_pixel(x, y + radius, color.clone());
    img.set_pixel(x, y - radius, color.clone());  
}

fn set_pixel(x: i32, y: i32, color: Pixel, wrap: bool, img: &mut FImage) {
    if wrap || (x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32) {
        img.set_pixel(x, y, color);
    }
}

fn set_pixel_blended(x: i32, y: i32, color: Pixel, wrap: bool, img: &mut FImage) {
    if wrap || (x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32) {
        img.set_pixel_blended(x, y, color);
    }
}

pub fn fill_circle(x: i32, y: i32, radius: i32, color: Pixel, wrap: bool, img: &mut FImage) {
    for i in 0..=radius {
        for j in 0..=radius {
            let d = ((i*i + j*j) as f32).sqrt();
            let diff = d - radius as f32;
            if diff < 0.0 {
                set_pixel(x + i, y + j, color.clone(), wrap, img);
                set_pixel(x - i, y + j, color.clone(), wrap, img);
                set_pixel(x + i, y - j, color.clone(), wrap, img);
                set_pixel(x - i, y - j, color.clone(), wrap, img);
            } else if diff < 1.0 {
                let newcolor = Pixel::rgba(color.r(), color.g(), color.b(), 1.0 - diff);

                set_pixel_blended(x + i, y + j, newcolor.clone(), wrap, img);
                set_pixel_blended(x - i, y + j, newcolor.clone(), wrap, img);
                set_pixel_blended(x + i, y - j, newcolor.clone(), wrap, img);
                set_pixel_blended(x - i, y - j, newcolor, wrap, img);
            }
        }
    }
}