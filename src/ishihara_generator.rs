use std::{collections::HashSet, f32::consts::PI};
use rand;

pub struct Circle {
    pub x: usize,
    pub y: usize,
    pub radius: usize
}

struct CircleGrid {
    width: usize,
    height: usize,
    grid_width: usize,
    grid_height: usize,
    grid_size: usize,
    data: Box<[f32]>
}

impl CircleGrid {
    pub fn new(width: usize, height: usize, grid_size: usize) -> CircleGrid {
        let grid_width = (width as f32 / grid_size as f32).ceil() as usize;
        let grid_height = (height as f32 / grid_size as f32).ceil() as usize;
        CircleGrid { width, height, grid_width, grid_height, grid_size, data: vec![-1.0; grid_width * grid_height].into_boxed_slice() }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[x / self.grid_size + (y / self.grid_size) * self.grid_width]
    }

    pub fn set(&mut self, x: usize, y: usize, value: f32) {
        self.data[x / self.grid_size + (y / self.grid_size) * self.grid_width] = value;
    }

    pub fn set_grid(&mut self, gx: usize, gy: usize, value: f32) {
        self.data[gx + gy * self.grid_width] = value;
    }

    pub fn get_grid(&mut self, gx: usize, gy: usize) -> f32 {
        self.data[gx + gy * self.grid_width]
    }

    pub fn grid_width(&self) -> usize {
        self.grid_width
    }

    pub fn grid_height(&self) -> usize {
        self.grid_height
    }

    pub fn grid_size(&self) -> usize {
        self.grid_size
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

fn pick_point(grid: &CircleGrid, set: &HashSet<(usize, usize)>) -> (usize, usize) {
    let choice = rand::random::<usize>() % set.len();

    let (gx, gy) = set.iter().nth(choice).unwrap().to_owned();
    let ox: f32 = rand::random();
    let oy: f32 = rand::random();

    let x = (gx as f32 + ox) * grid.grid_size() as f32;
    let y = (gy as f32 + oy) * grid.grid_size() as f32;

    (x as usize, y as usize)
}

pub fn generate_circles(width: usize, height: usize, min_radius: usize, max_radius: usize, padding: usize, coverage: f32) -> Box<[Circle]> {
    let mut circles = Vec::new();


    let mut grid = CircleGrid::new(width, height, min_radius / 2);

    let mut area = 0.0;
    let total_area = (width * height) as f32;

    let mut available_spaces = HashSet::new();

    for x in 0..grid.grid_width() {
        for y in 0..grid.grid_height() {
            available_spaces.insert((x, y));
        }
    }

    while area < total_area * coverage {
        let mut max = max_radius;

        if available_spaces.len() == 0 {
            println!("Ran out of space!");
            break;
        }

        let (x, y) = pick_point(&grid, &available_spaces);

        let dist = grid.get(x, y);
        if dist != -1.0 {
            max = dist as usize - padding;
        }

        let radius = rand::random::<usize>() % (max - min_radius) + min_radius;
        area += PI * (radius * radius) as f32;

        circles.push(Circle { x, y, radius });

        //update grid spaces
        let start_grid = (x / grid.grid_size, y / grid.grid_size);

        let mut visited = HashSet::new();
        let mut q = vec![start_grid];
        while q.len() > 0 {
            let (px, py) = q.pop().unwrap();
            if !visited.contains(&(px, py)) {
                visited.insert((px, py));
                let rpx = (px as f32 + 0.5) * grid.grid_size() as f32;
                let rpy = (py as f32 + 0.5) * grid.grid_size() as f32;
                let dist = (((x as i32 - rpx as i32).pow(2) + (y as i32 - rpy as i32).pow(2)) as f32).sqrt() - radius as f32;

                if dist <= (max_radius + padding) as f32 {
                    let c = grid.get_grid(px, py);
                    if c == -1.0 || dist < c {
                        grid.set_grid(px, py, dist);
                    }

                    if px + 1 < grid.grid_width() {
                        q.push((px + 1, py));
                    }
                    if py + 1 < grid.grid_height() {
                        q.push((px, py + 1));
                    }
                    if px > 0 {
                        q.push((px - 1, py));
                    }
                    if py > 0 {
                        q.push((px, py - 1));
                    }

                    // the plus 1 is t make sure 
                    if dist < (min_radius + padding + 1) as f32 {
                        available_spaces.remove(&(px, py));
                    }
                }
            }
        }
    }

    circles.into_boxed_slice()
}