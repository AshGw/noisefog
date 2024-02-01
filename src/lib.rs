pub mod funcs;
use funcs::{euclidean_distance,lerp,invlerp,clamp};
use wasm_bindgen::prelude::*;

use noise::{Seedable, NoiseFn, Perlin};
use lerp::Lerp;


#[wasm_bindgen]
pub struct NoiseMap {
    width: usize,
    height: usize,
    scale: f64,
    octaves: u8,
    lacunarity: f64,
    persistence: f64,
    max_value: f64,
    min_value: f64,
    map: Vec<f64>
}


#[wasm_bindgen]
impl NoiseMap {
    pub fn new(
        width: usize,
        height: usize,
        scale: f64,
        octaves: u8,
        lacunarity: f64,
        persistence: f64,
        reshape: bool,
    ) -> NoiseMap {

        let (center_x, center_y) = (width as f64 / 2.0, height as f64 / 2.0);
        let max_distance_from_center = euclidean_distance((0.0, 0.0),
            (center_x, center_y));

        let noise = Perlin::new();
        noise.set_seed(rand::random());

        let mut noise_map = Vec::with_capacity(height);
        let mut max_value = std::f64::MIN;
        let mut min_value = std::f64::MAX;

        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let mut noise_val = 0_f64;
                for octave_idx in 0..octaves {
                    let octave_idx = octave_idx as i32;
                    let sample_x = x as f64 / scale * lacunarity.powi(octave_idx);
                    let sample_y = y as f64 / scale * lacunarity.powi(octave_idx);
                    noise_val += noise.get([sample_x, sample_y]) * persistence.powi(octave_idx);
                }

                if reshape {
                    let distance_to_map_center = euclidean_distance((x as f64, y as f64),
                      (center_x, center_y));
                    let d = invlerp(0.0, max_distance_from_center, distance_to_map_center);
                    let d = -lerp(-1.0, 1.0, d);
                    noise_val = clamp(noise_val + d, 0.0, 1.0);
                }

                row.push(noise_val);

                if noise_val > max_value {
                    max_value = noise_val;
                } else if noise_val < min_value {
                    min_value = noise_val;
                }
            }

            noise_map.push(row);
        }

        NoiseMap {
            width,
            height,
            scale,
            octaves,
            lacunarity,
            persistence,
            max_value,
            min_value,
            map: noise_map.concat() // flatten the 2D into a 1D for WASM interop.
        }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn scale(&self) -> f64 { self.scale }
    pub fn octaves(&self) -> u8 { self.octaves }
    pub fn lacunarity(&self) -> f64 { self.lacunarity }
    pub fn persistence(&self) -> f64 { self.persistence }
    pub fn noise_map(&self) -> *const f64 { self.map.as_ptr() }
    pub fn max_value(&self) -> f64 { self.max_value }
    pub fn min_value(&self) -> f64 { self.min_value }
}

pub fn write_grid_to_file(grid: &[Vec<f64>], path: &str) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    let png_encoder = image::png::PNGEncoder::new(file);
    let width = grid[0].len();
    let height = grid.len();
    let mut lerp_grid = Vec::with_capacity(height);
    for i in 0..height {
        let mut row = Vec::with_capacity(width);
        for j in 0..width {
            let val = 128_f64.lerp(255.0, grid[j][i]);
            row.push(val as u8);
        }
        lerp_grid.push(row);
    }
    png_encoder.encode(&lerp_grid.concat(),
                       grid[0].len() as u32,
                       grid.len() as u32,
                       image::ColorType::Gray(8))
}