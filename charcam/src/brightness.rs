pub fn brightness(r: f32, g: f32, b: f32) -> f32 {
    (0.299 * r + 0.587 * g + 0.114 * b) / 255.0
}
