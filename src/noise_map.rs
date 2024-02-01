use wasm_bindgen::prelude::*;
use std::marker::Copy; 

#[wasm_bindgen]
pub struct NoiseMap {
    pub width: usize,
    pub height: usize,
    pub scale: f64,
    pub octaves: u8,
    pub lacunarity: f64,
    pub persistence: f64,
    pub max_value: f64,
    pub min_value: f64,
    #[wasm_bindgen(skip)]
    pub map: Vec<f64>
}

