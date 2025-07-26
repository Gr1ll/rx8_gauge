use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use framebuffer::Framebuffer;
use std::io;
use std::path::Path;

const WIDTH: usize = 480;
const HEIGHT: usize = 320;

pub struct FbDisplay {
    fb: Framebuffer,
    buffer: Vec<u8>,
}

impl FbDisplay {
    pub fn new() -> io::Result<Self> {
        let fb = Framebuffer::new(Path::new("/dev/fb0"))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{e}")))?;

        let buffer = vec![0; WIDTH * HEIGHT * 2];

        Ok(Self { fb, buffer })
    }

    pub fn flush(&mut self) {
        self.fb.write_frame(&self.buffer);
    }
}

impl OriginDimensions for FbDisplay {
    fn size(&self) -> Size {
        Size::new(WIDTH as u32, HEIGHT as u32)
    }
}

impl DrawTarget for FbDisplay {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Rgb565>>,
    {
        for Pixel(coord, color) in pixels {
            let x = coord.x;
            let y = coord.y;

            if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                let x = x as usize;
                let y = y as usize;
                let idx = (y * WIDTH + x) * 2;

                let raw: u16 = color.into_storage();
                self.buffer[idx] = (raw & 0xFF) as u8;
                self.buffer[idx + 1] = (raw >> 8) as u8;
            }
        }

        Ok(())
    }
}
