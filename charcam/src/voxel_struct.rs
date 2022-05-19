pub struct Voxel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub c: char,
}

impl Voxel {
    pub fn new(r: u8, g: u8, b: u8) -> Voxel {
        let bright = super::brightness::brightness(r, g, b);
        let bright_char = super::chars::density_char(&bright);
        Voxel {
            r,
            g,
            b,
            c: bright_char,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_manual_voxel() {
        let voxel = Voxel {
            r: 0,
            g: 255,
            b: 0,
            c: 't',
        };
        assert_eq!(voxel.r, 0);
        assert_eq!(voxel.g, 255);
        assert_eq!(voxel.b, 0);
        assert_eq!(voxel.c, 't');
    }

    #[test]
    fn create_voxel_with_new() {
        let voxel = Voxel::new(0, 255, 0);
        assert_eq!(voxel.r, 0);
        assert_eq!(voxel.g, 255);
        assert_eq!(voxel.b, 0);
        assert_eq!(voxel.c, '2');
    }
}
