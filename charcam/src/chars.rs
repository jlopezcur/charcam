pub fn density_char(factor: &f32) -> char {
    let density = " _.,-=+:;cba!?0123456789$W#@Ñ";
    let index = (factor * (density.chars().count() - 1) as f32) as usize;
    density.chars().nth(index).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_density_char_cases() {
        assert_eq!(density_char(&0.0), ' ');
        assert_eq!(density_char(&0.5), '0');
        assert_eq!(density_char(&0.9), 'W');
        assert_eq!(density_char(&0.95), '#');
        assert_eq!(density_char(&0.99), '@');
        assert_eq!(density_char(&1.0), 'Ñ');
    }
}
