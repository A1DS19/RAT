use image::load_from_memory;
use opencv::core::Mat;
use opencv::highgui;
use opencv::imgproc;
use opencv::prelude::*;
use opencv::Error;
use tracing::info;

pub fn display_video_feed(encoded_buffer: Vec<u8>) -> Result<(), Error> {
    info!(
        "Displaying video feed with buffer length: {}",
        encoded_buffer.len()
    );

    let img = load_from_memory(&encoded_buffer)
        .expect("Failed to decode image from memory")
        .to_rgba8();

    let (width, height) = img.dimensions();

    let mat = Mat::from_slice(img.as_raw()).expect("Failed to create Mat from slice");

    let mat = mat
        .reshape(4, height as i32)
        .expect("Failed to reshape Mat");

    let mut bgr_mat = Mat::default();
    imgproc::cvt_color(&mat, &mut bgr_mat, imgproc::COLOR_RGBA2BGR, 0)?;

    highgui::imshow("Screen Capture", &bgr_mat)?;

    highgui::wait_key(0)?;

    Ok(())
}
