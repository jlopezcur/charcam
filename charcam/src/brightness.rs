pub fn brightness(r: u8, g: u8, b: u8) -> f32 {
    (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_brightness_cases() {
        assert_eq!(brightness(0, 0, 0), 0.0);
        assert_eq!(brightness(255, 255, 255), 1.0);
        assert_eq!(brightness(128, 128, 128), 0.5019608);
    }
}
