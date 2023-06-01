use image::{GenericImage, Primitive};

const SAMPLE_X: usize = 977;
const SAMPLE_Y: usize = 414;

#[derive(Debug)]
pub enum Pixel {
    RGB{r: f32, g: f32, b: f32},
    RGBA{r: f32, g: f32, b: f32, a: f32}
}

#[derive(Clone)]
pub struct FImage {
    width: usize,
    height: usize,
    alpha: bool,
    pixels: Box<[f32]>
}

impl FImage {
    pub fn new(width: usize, height: usize, use_alpha_channel: bool) -> FImage {
        let channels = if use_alpha_channel {4} else {3};
        let data = vec![0.0; width * height * channels];

        FImage { width, height, alpha: use_alpha_channel, pixels: data.into_boxed_slice() }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        let channels = if self.alpha {4} else {3};
        
        let mod_x = {let r = x % self.width as i32; if r < 0 {r + self.width as i32} else {r}} as usize;
        let mod_y = {let r = y % self.height as i32; if r < 0 {r + self.height as i32} else {r}} as usize;

        let offset = channels as usize * (mod_x + mod_y * self.width);

        if self.alpha {
            Pixel::RGBA{r: self.pixels[offset], g: self.pixels[offset + 1], b: self.pixels[offset + 2], a: self.pixels[offset + 3]}
        } else {
            Pixel::RGB{r: self.pixels[offset], g: self.pixels[offset + 1], b: self.pixels[offset + 2]}
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, pixel: Pixel) {
        let channels = if self.alpha {4} else {3};
        
        let mod_x = {let r = x % self.width as i32; if r < 0 {r + self.width as i32} else {r}} as usize;
        let mod_y = {let r = y % self.height as i32; if r < 0 {r + self.height as i32} else {r}} as usize;

        // println!("x: {}\t mod_x: {}", x, mod_x);
        // println!("y: {}\t mod_y: {}", y, mod_y);

        let offset = channels as usize * (mod_x + mod_y * self.width);

        if self.alpha {
            let p = match pixel { Pixel::RGBA { r, g, b, a } => (r, g, b, a), _ => panic!("Incorrect pixel format!")};

            self.pixels[offset] = p.0;
            self.pixels[offset + 1] = p.1;
            self.pixels[offset + 2] = p.2;
            self.pixels[offset + 3] = p.3;
        } else {
            let p = match pixel { Pixel::RGB { r, g, b} => (r, g, b), _ => panic!("Incorrect pixel format!")};

            self.pixels[offset] = p.0;
            self.pixels[offset + 1] = p.1;
            self.pixels[offset + 2] = p.2;
        }
    }

    pub fn has_alpha_channel(&self) -> bool {
        self.alpha
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn normalize(&mut self) {
        let mut min = f32::INFINITY;
        let mut max = f32::NEG_INFINITY;

        for (i, c) in self.pixels.iter().enumerate() {
            if !self.alpha || i % 4 != 3 {
                if *c > max {max = *c}
                if *c < min {min = *c}
            }
        }

        println!("min: {}, max: {}", min, max);

        self.clip(min, max);
    }

    pub fn clip(&mut self, min: f32, max: f32) {
        let range = max - min;
        for (i, c) in self.pixels.iter_mut().enumerate() {  
            if !self.alpha || i % 4 != 3 {
                *c = ((*c - min) / range).clamp(0.0, 1.0);
            }
        }
    }

    pub fn copy_from_image_buffer<SP: Primitive, P: image::Pixel<Subpixel = SP>, I: GenericImage<Pixel = P>>(&mut self, image: &I) {
        if image.width() as usize != self.width || image.height() as usize != self.height {
            panic!("Dimensions do not match!");
        }

        print!("Before copy from: ");
        println!("Pixel read at ({}, {}): {:?}", SAMPLE_X, SAMPLE_Y, self.get_pixel(SAMPLE_X as i32, SAMPLE_Y as i32));

        for x in 0..self.width {
            for y in 0..self.height {
                let p = if self.alpha {
                    let t = image.get_pixel(x as u32, y as u32).to_rgba();
                    Pixel::RGBA { r: t.0[0].to_f32().unwrap(), g: t.0[1].to_f32().unwrap(), b: t.0[2].to_f32().unwrap(), a: t.0[3].to_f32().unwrap() }
                } else {
                    let t = image.get_pixel(x as u32, y as u32).to_rgb();
                    Pixel::RGB { r: t.0[0].to_f32().unwrap(), g: t.0[1].to_f32().unwrap(), b: t.0[2].to_f32().unwrap() }
                };

                self.set_pixel(x as i32, y as i32, p);

                if x == SAMPLE_X && y == SAMPLE_Y {
                    print!("During copy from: ");
                    println!("Pixel read at ({}, {}): {:?}", x, y, self.get_pixel(x as i32, y as i32));
                }
            }
        }

        print!("After copy from: ");
        println!("Pixel read at ({}, {}): {:?}", SAMPLE_X, SAMPLE_Y, self.get_pixel(SAMPLE_X as i32, SAMPLE_Y as i32));
    }

    pub fn copy_to_image_buffer<SP: Primitive, P: image::Pixel<Subpixel = SP>, I: GenericImage<Pixel = P>>(&self, image: &mut I) {
        if image.width() as usize != self.width || image.height() as usize != self.height {
            panic!("Dimensions do not match!");
        }

        let mut temp = self.clone();
        temp.normalize();

        for x in 0..self.width {
            for y in 0..self.height {
                let p = temp.get_pixel(x as i32, y as i32);
                if x == SAMPLE_X && y == SAMPLE_Y {
                    println!("Pixel read at ({}, {}): {:?}", SAMPLE_X, SAMPLE_Y, temp.get_pixel(x as i32, y as i32));
                }

                let mut v = vec![SP::DEFAULT_MAX_VALUE; P::CHANNEL_COUNT as usize];
                let a = match p {
                    Pixel::RGB { r, g, b } => vec![r, g, b],
                    Pixel::RGBA { r, g, b, a } => vec![r, g, b, a]
                };

                for i in 0..v.len().min(a.len()) {
                    v[i] = SP::from(a[i] * 255.0).unwrap();
                }

                let pixel = P::from_slice(&v).clone();

                image.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }
}