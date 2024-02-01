use wasm_bindgen::prelude::*;

type Float64Tuple = (f64, f64);
pub fn euclidean_distance(p1: Float64Tuple, p2: Float64Tuple) -> f64 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}
#[wasm_bindgen]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}
#[wasm_bindgen]
pub fn invlerp(a: f64, b: f64, v: f64) -> f64 {
    clamp((v - a) / (b - a), 0.0, 1.0)
}
#[wasm_bindgen]
pub fn clamp(v: f64, min: f64, max: f64) -> f64 {
    max.min(min.max(v))
}
