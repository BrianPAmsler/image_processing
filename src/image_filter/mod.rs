mod filter_matrix;
use std::collections::HashMap;

pub use filter_matrix::FilterMatrix;

use crate::float_image::{Pixel, FImage, PixelFormat};
use priority_queue::PriorityQueue;

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

pub fn fn_filter<FN: FnMut(i32, i32, Pixel) -> Pixel>(img: &FImage, mut func: FN) -> FImage {
    let mut out = FImage::new(img.width(), img.height(), img.get_pixel_format());
    
    for x in 0..img.width() {
        for y in 0..img.height() {
            let px = img.get_pixel(x as i32, y as i32);

            out.set_pixel(x as i32, y as i32, func(x as i32, y as i32, px));
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

#[derive(PartialOrd, PartialEq)]
struct OF32(f32);

impl Eq for OF32 {}

impl Ord for OF32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
	if let Some(ordering) = self.partial_cmp(other) {
	    ordering
	} else {
	    // Choose what to do with NaNs, for example:
	    std::cmp::Ordering::Less
	}
    }
}

fn get_label(x: i32, y: i32, map: &HashMap<(i32, i32), i32>) -> i32 {
    match map.get(&(x, y)) {
        Some(s) => *s,
        None => 0
    }
}

pub fn watershed(img: &FImage, p1: (i32, i32), p2: (i32, i32)) -> FImage {
    if !matches!(img.get_pixel_format(), PixelFormat::Mono) {
        panic!("Image must be mono for watershed!");
    }

    let mut out = FImage::new(img.width(), img.height(), PixelFormat::Mono);

    let mut pq = PriorityQueue::new();
    let mut labels = HashMap::new();

    pq.push(p1, OF32(img.get_pixel(p1.0, p1.1).r()));
    pq.push(p2, OF32(img.get_pixel(p2.0, p2.1).r()));

    labels.insert(p1, 1);
    labels.insert(p2, 2);

    let w = img.width() as i32;
    let h = img.height() as i32;
    while pq.len() > 0 {
        // The pixel with the highest priority level is extracted from the priority queue. If the neighbors of the extracted pixel that have already been labeled all have the same label, then the pixel is labeled with their label. All non-marked neighbors that are not yet in the priority queue are put into the priority queue.
        let pixel = pq.pop().unwrap();
        let neighbors = [(pixel.0.0, (pixel.0.1 + 1) % h), (pixel.0.0, (pixel.0.1 + h - 1) % h), ((pixel.0.0 + w - 1) % w, pixel.0.1), ((pixel.0.0 + 1) % w, pixel.0.1)];

        let mut labeled = Vec::new(); labeled.reserve(4);
        let mut unlabeled = Vec::new(); unlabeled.reserve(4);

        for p in neighbors {
            let label = get_label(p.0, p.1, &labels);
            if label > 0 {
                labeled.push((p, label));
            } else if label == 0 {
                unlabeled.push(p);
            }
        }

        // println!("lableled: {:?}", labeled);
        // println!("unlabled: {:?}", unlabeled);

        let mut first_label = labeled.get(0).map(|t| t.1);
        if labeled.len() > 1 {
            for p in &labeled[1..] {
                if p.1 != first_label.unwrap() {
                    first_label = None;
                    break;
                }
            }
        }

        // first_label will be None if either there are no labeled pixels or the labeled pixels have different labels
        if first_label.is_some() {
            labels.insert((pixel.0.0, pixel.0.1), first_label.unwrap());
            // println!("lableled: {}", first_label.unwrap());
        } else if !labels.contains_key(&(pixel.0.0, pixel.0.1)){
            labels.insert((pixel.0.0, pixel.0.1), -1);
        }

        for p in unlabeled {
            pq.push(p, OF32(img.get_pixel(p.0, p.1).r()));
        }

        // panic!("test");
    }

    for x in 0..img.width() as i32 {
        for y in 0.. img.height() as i32 {
            let pixel = match get_label(x, y, &labels) {
                2 | 0 | -1 => Pixel::mono(0.0),
                1 => Pixel::mono(1.0),
                _ => panic!("bork")
            };

            out.set_pixel(x, y, pixel);
        }
    }

    out
}