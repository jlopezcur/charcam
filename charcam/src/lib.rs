pub fn density_char(factor: &f32) -> char {
    let density = " _.,-=+:;cba!?0123456789$W#@Ñ";
    let index = (factor * (density.chars().count() - 1) as f32) as usize;
    density.chars().nth(index).unwrap()
}

pub fn brightness(r: f32, g: f32, b: f32) -> f32 {
    (0.299 * r + 0.587 * g + 0.114 * b) / 255.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(density_char(&0.0), ' ');
        assert_eq!(density_char(&0.5), '0');
        assert_eq!(density_char(&0.9), 'W');
        assert_eq!(density_char(&0.95), '#');
        assert_eq!(density_char(&0.99), '@');
        assert_eq!(density_char(&1.0), 'Ñ');
    }
}
