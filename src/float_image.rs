use image::{GenericImage, Primitive};

const SAMPLE_X: usize = 977;
const SAMPLE_Y: usize = 414;

#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    RGB,
    RGBA,
    Mono
}

impl PixelFormat {
    pub fn channel_count(&self) -> usize {
        match self { PixelFormat::Mono => 1, PixelFormat::RGB => 3, PixelFormat::RGBA => 4 }
    }
}

#[derive(Debug, Clone)]
enum PixelData<'p> {
    Owned(Box<[f32]>),
    Slice(&'p [f32])
}

#[derive(Debug)]
pub struct Pixel<'p> {
    data: PixelData<'p>,
    format: PixelFormat
}

impl Pixel<'_> {
    pub fn rgb<'p>(r: f32, g: f32, b: f32) -> Pixel<'p> {
        let data = Box::new([r, g, b]);
        Pixel { data: PixelData::Owned(data), format: PixelFormat::RGB }
    }

    pub fn rgba<'p>(r: f32, g: f32, b: f32, a: f32) -> Pixel<'p> {
        let data = Box::new([r, g, b, a]);
        Pixel { data: PixelData::Owned(data), format: PixelFormat::RGBA }
    }

    pub fn mono<'p>(r: f32) -> Pixel<'p> {
        let data = Box::new([r]);
        Pixel { data: PixelData::Owned(data), format: PixelFormat::Mono }
    }

    pub fn from_slice<'p>(slice: &'p [f32]) -> Pixel<'p> {
        let format = match slice.len() {
            1 => PixelFormat::Mono,
            3 => PixelFormat::RGB,
            4 => PixelFormat::RGBA,
            _ => panic!("Invalid pixel slice!")
        };

        Pixel { data: PixelData::Slice(slice), format }
    }

    pub fn from_boxed_slice<'p>(bx: Box<[f32]>) -> Pixel<'p> {
        let format = match bx.len() {
            1 => PixelFormat::Mono,
            3 => PixelFormat::RGB,
            4 => PixelFormat::RGBA,
            _ => panic!("Invalid pixel slice!")
        };

        Pixel { data: PixelData::Owned(bx), format }
    }

    pub fn slice<'p>(&'p self) -> &'p [f32] {
        match &self.data {
            PixelData::Owned(d) => &d[..],
            PixelData::Slice(s) => *s,
        }
    }

    pub fn r(&self) -> f32 {
        self.slice()[0]
    }

    pub fn g(&self) -> f32 {
        match self.slice().len() {
            1 => self.slice()[0],
            _ => self.slice()[1]
        }
    }

    pub fn b(&self) -> f32 {
        match self.slice().len() {
            1 => self.slice()[0],
            _ => self.slice()[2]
        }
    }

    pub fn a(&self) -> f32 {
        match self.slice().len() {
            4 => self.slice()[3],
            _ => 1.0
        }
    }

    pub fn format(&self) -> PixelFormat {
        self.format
    }
}

impl<'p> Clone for Pixel<'p> {
    fn clone(&self) -> Pixel<'p> {
        let slice = self.slice();
        let data = slice.to_owned();
        Pixel { data: PixelData::Owned(data.into_boxed_slice()), format: self.format }
    }
}

#[derive(Clone)]
pub struct FImage {
    width: usize,
    height: usize,
    format: PixelFormat,
    pixels: Box<[f32]>
}

impl FImage {
    pub fn new(width: usize, height: usize, format: PixelFormat) -> FImage {
        let channels = format.channel_count();
        let data = vec![0.0; width * height * channels];

        FImage { width, height, format, pixels: data.into_boxed_slice() }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        let channels = self.format.channel_count();
        
        let mod_x = {let r = x % self.width as i32; if r < 0 {r + self.width as i32} else {r}} as usize;
        let mod_y = {let r = y % self.height as i32; if r < 0 {r + self.height as i32} else {r}} as usize;

        let offset = channels as usize * (mod_x + mod_y * self.width);

        Pixel::from_slice(&self.pixels[offset..offset + channels])
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, pixel: Pixel) {
        let channels = self.format.channel_count();
        
        let mod_x = {let r = x % self.width as i32; if r < 0 {r + self.width as i32} else {r}} as usize;
        let mod_y = {let r = y % self.height as i32; if r < 0 {r + self.height as i32} else {r}} as usize;

        let offset = channels as usize * (mod_x + mod_y * self.width);

        self.pixels[offset] = pixel.r();
        
        if channels >= 3 {
            self.pixels[offset + 1] = pixel.g();
            self.pixels[offset + 2] = pixel.b();
        }

        if channels == 4 {
            self.pixels[offset + 3] = pixel.a();
        }
    }

    pub fn set_pixel_blended(&mut self, x: i32, y: i32, pixel: Pixel) {
        let current = self.get_pixel(x, y);
        
        let a = pixel.a();
        let a_ = 1.0 - a;

        // TODO: This does not work properly if the current pixel has alpha
        let new_pixel = Pixel::rgba(pixel.r() * a + current.r() * a_, pixel.g() * a + current.g() * a_, pixel.b() * a + current.b() * a_, current.a());

        self.set_pixel(x, y, new_pixel);
    }

    pub fn get_pixel_format(&self) -> PixelFormat {
        self.format
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
            if !matches!(self.format, PixelFormat::RGBA) || i % 4 != 3 {
                if *c > max {max = *c}
                if *c < min {min = *c}
            }
        }

        println!("min: {}, max: {}", min, max);

        self.clip(min, max);
    }

    pub fn clip(&mut self, min: f32, max: f32) {
        let mut range = max - min;
        if range == 0.0 {
            range = 1.0;
        }
        for (i, c) in self.pixels.iter_mut().enumerate() {  
            if !matches!(self.format, PixelFormat::RGBA) || i % 4 != 3 {
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
                let t = image.get_pixel(x as u32, y as u32).to_rgba();
                let p = Pixel::rgba(t.0[0].to_f32().unwrap() / 255.0, t.0[1].to_f32().unwrap() / 255.0, t.0[2].to_f32().unwrap() / 255.0, t.0[3].to_f32().unwrap() / 255.0);

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
        temp.clip(0.0, 1.0);

        for x in 0..self.width {
            for y in 0..self.height {
                let p = temp.get_pixel(x as i32, y as i32);
                let p2 = Pixel::rgba(p.r(), p.g(), p.b(), p.a());

                let mut v = vec![SP::DEFAULT_MAX_VALUE; P::CHANNEL_COUNT as usize];
                let a = p2.slice();

                for i in 0..v.len().min(a.len()) {
                    v[i] = SP::from(a[i] * 255.0).unwrap();
                }

                let pixel = P::from_slice(&v).clone();

                if x == SAMPLE_X && y == SAMPLE_Y {
                    let colorpx = Pixel::rgb(p.r(), p.g(), p.b());
                    println!("Pixel read at ({}, {}): {:?}", SAMPLE_X, SAMPLE_Y, colorpx);

                    println!("Pixel put at ({}, {}): ({}, {}, {})", SAMPLE_X, SAMPLE_Y, a[0], a[1], a[2]);
                }

                image.put_pixel(x as u32, y as u32, pixel);
            }
        }
    }
}