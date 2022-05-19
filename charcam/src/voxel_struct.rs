pub struct Voxel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub c: char,
}

impl Voxel {
    pub fn new(r: u8, g: u8, b: u8) -> Voxel {
        let bright = super::brightness::brightness(r as f32, g as f32, b as f32);
        let bright_char = super::chars::density_char(&bright);
        Voxel {
            r,
            g,
            b,
            c: bright_char,
        }
    }
}
