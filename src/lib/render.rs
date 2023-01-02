use std::slice::{ChunksExact, ChunksExactMut};

pub enum PixelFormat {
    RGBA
}

pub const fn bytes_per_pixel(fmt: PixelFormat) -> usize {
    match fmt {
        PixelFormat::RGBA => 4
    }
}

pub const PIXEL_FORMAT: PixelFormat = PixelFormat::RGBA;
pub const BYTES_PER_PIXEL: usize = bytes_per_pixel(PIXEL_FORMAT);

pub struct Pixels {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize
}
impl Pixels {
    pub fn ind(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y < self.height);
        BYTES_PER_PIXEL * (y * self.width + x)
    }
    /// Iterate over pixel data by location in row-fastest order.
    pub fn iter_row_col(&self) -> ChunksExact<u8> {
        self.data.chunks_exact(BYTES_PER_PIXEL)
    }
    /// Iterate over mutable pixel data by location in row-fastest order.
    pub fn iter_row_col_mut(&mut self) -> ChunksExactMut<u8> {
        self.data.chunks_exact_mut(BYTES_PER_PIXEL)
    }
}

pub fn fill_pix(pix: &mut [u8], color: Color) {
    match PIXEL_FORMAT {
        PixelFormat::RGBA => {
            assert!(pix.len() == 4);
            let ptr = pix.as_mut_ptr();
            unsafe {
                *ptr = (255.0 * color.r) as u8;
                *ptr.add(1) = (255.0 * color.g) as u8;
                *ptr.add(2) = (255.0 * color.b) as u8;
                *ptr.add(3) = (255.0 * color.a) as u8;
            }
        }
    };
}

/// Universal representation of color, to be converted into specific binary
/// format during rendering. Each of channel lives in the interval [0.0, 1.0].
#[derive(Clone,Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}
impl Color {
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new_rgba(r, g, b, 1.0)
    }
    pub fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        assert!(0.0 <= r && r <= 1.0);
        assert!(0.0 <= g && g <= 1.0);
        assert!(0.0 <= b && b <= 1.0);
        assert!(0.0 <= a && a <= 1.0);
        Self { r: r, g: g, b: b, a: a }
    }
}

pub const EMPTY_COLOR: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };