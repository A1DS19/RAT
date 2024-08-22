use image::codecs::png::PngEncoder;
use image::{ImageBuffer, Rgba};
use image::{ImageEncoder, PixelWithColorType};
use scrap::{Capturer, Display};
use std::io::Cursor;
use std::io::ErrorKind;
use std::thread;
use std::time::Duration;
use tracing::info;

pub fn see_screen() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let display = Display::primary().expect("Failed to get primary display");
    let mut capturer = Capturer::new(display).expect("Failed to create capturer");

    let width = capturer.width();
    let height = capturer.height();

    info!("Capturing screen at {}x{}", width, height);

    loop {
        match capturer.frame() {
            Ok(frame) => {
                let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
                    ImageBuffer::from_raw(width as u32, height as u32, frame.to_vec())
                        .expect("Failed to create ImageBuffer");

                let mut encoded_buffer = Vec::new();
                let mut cursor = Cursor::new(&mut encoded_buffer);
                let encoder = PngEncoder::new(&mut cursor);

                encoder.write_image(
                    img.as_raw(),
                    width as u32,
                    height as u32,
                    Rgba::<u8>::COLOR_TYPE,
                )?;

                info!("Screen captured buffer: {}", encoded_buffer.len());

                return Ok(encoded_buffer);
            }
            Err(error) => {
                if error.kind() == ErrorKind::WouldBlock {
                    thread::sleep(Duration::from_millis(10));
                    continue;
                } else {
                    return Err(Box::new(error));
                }
            }
        }
    }
}
