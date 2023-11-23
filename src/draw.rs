use pixels::Pixels;
use image::{GenericImageView, DynamicImage, RgbaImage};

pub fn draw(image_data: &Vec<Vec<(u8, u8, u8, u8)>>, pixels: &mut Pixels, width: u32, height: u32) {
    let mut frame = pixels.get_frame_mut();

    for (y, row) in image_data.iter().enumerate() {
        for (x, &(r, g, b, a)) in row.iter().enumerate() {
            let pixel_index = (y * width as usize + x) * 4;
            if pixel_index < frame.len() {
                frame[pixel_index] = r;
                frame[pixel_index + 1] = g;
                frame[pixel_index + 2] = b;
                frame[pixel_index + 3] = a;
            }
        }
    }

    pixels.render().expect("Failed to render pixels");
}

pub fn draw_scaled(image_data: &Vec<Vec<(u8, u8, u8, u8)>>, pixels: &mut Pixels, window_width: u32, window_height: u32) {
    let mut frame = pixels.get_frame_mut();

    // Set the entire frame to black
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0, 0, 0, 255]); // RGBA for black
    }

    // Handle empty image data
    if image_data.is_empty() || image_data[0].is_empty() {
        return;
    }

    let image_width = image_data[0].len() as f32;
    let image_height = image_data.len() as f32;

    // Calculate scale factors while maintaining aspect ratio
    let scale = (window_width as f32 / image_width).min(window_height as f32 / image_height);

    // Calculate the new image dimensions
    let scaled_width = (image_width * scale) as u32;
    let scaled_height = (image_height * scale) as u32;

    // Calculate offset to center the image
    let x_offset = (window_width - scaled_width) / 2;
    let y_offset = (window_height - scaled_height) / 2;

    for y in 0..window_height {
        for x in 0..window_width {
            let pixel_index = (y as usize * window_width as usize + x as usize) * 4;

            // Ensure the pixel index is within the frame buffer
            if pixel_index + 3 < frame.len() {
                if x >= x_offset && x < x_offset + scaled_width && y >= y_offset && y < y_offset + scaled_height {
                    // Adjust x and y to account for the offset
                    let adjusted_x = x - x_offset;
                    let adjusted_y = y - y_offset;

                    // Determine which pixel in the original image corresponds to this pixel
                    let src_x = (adjusted_x as f32 / scale) as usize;
                    let src_y = (adjusted_y as f32 / scale) as usize;

                    let (r, g, b, a) = image_data[src_y][src_x];

                    frame[pixel_index] = r;
                    frame[pixel_index + 1] = g;
                    frame[pixel_index + 2] = b;
                    frame[pixel_index + 3] = a;
                } else {
                    // Set pixel to black if it's outside the scaled image area
                    frame[pixel_index] = 0;
                    frame[pixel_index + 1] = 0;
                    frame[pixel_index + 2] = 0;
                    frame[pixel_index + 3] = 255;
                }
            }
        }
    }

    pixels.render().expect("Failed to render pixels");
}
