use super::Pixel;

pub type RelPixel = [f64; 2];

pub trait RelPixelExt {
    fn to_absolute(&self, resolution: Pixel) -> Pixel;
    fn neighbours(&self, resolution: Pixel, extent: [f64; 2]) -> Vec<Pixel>;
}

impl RelPixelExt for RelPixel {
    fn to_absolute(&self, resolution: Pixel) -> Pixel {
        let mut result = [0, 0];
        for i in 0..2 {
            assert!(-0.5 < self[i] && self[i] < 0.5);
            result[i] = (resolution[i] as f64 * (self[i] + 0.5)) as u32;
        }
        result
    }

    fn neighbours(&self, resolution: Pixel, extent: [f64; 2]) -> Vec<Pixel> {
        assert!(extent[0] >= 0.0 && extent[1] >= 0.0);
        let mut lower = [0, 0];
        let mut upper = [0, 0];
        for i in 0..2 {
            lower[i] = ((self[i] - extent[i] + 0.5) * (resolution[i] as f64)).floor() as u32;
            upper[i] = ((self[i] + extent[i] + 0.5) * (resolution[i] as f64)).ceil() as u32;
        }
        let mut result = Vec::new();
        for x in lower[0]..upper[0] {
            for y in lower[1]..upper[1] {
                result.push([x, y]);
            }
        }

        result
    }
}

pub trait PixelExt {
    fn to_relative(&self, resolution: Pixel) -> RelPixel;
}

impl PixelExt for Pixel {
    fn to_relative(&self, resolution: Pixel) -> RelPixel {
        let mut result = [0.0, 0.0];
        for i in 0..2 {
            let res = resolution[i];
            assert!(self[i] < res);
            let pixel_width = 1.0 / (res as f64);
            result[i] = (((self[i] as f64) + 0.5) * pixel_width) - 0.5;
            assert!(-0.5 < result[i] && result[i] < 0.5);
        }
        result
    }
}
