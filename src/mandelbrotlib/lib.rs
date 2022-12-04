
use image::{self, RgbImage};

const MAX_ITERATION: i32 = 300;

pub fn set_rgb(iteration: i32) -> image::Rgb<u8> {
    if iteration > MAX_ITERATION {
        return image::Rgb([255, 255, 255]);
    }
    let col = (((iteration as f32) / (MAX_ITERATION as f32)) * 255.0).round() as u8;
    return image::Rgb([col, col, col]);
}

pub fn generate_image(width: u32, height: u32) -> image::RgbImage {
    let mut img = RgbImage::new(width, height);
    for w in 0..width - 100 {
        let x0 = ((w as f32) / (width as f32)) * 3.5 - 2.5;
        for h in 0..height - 100 {
            let mut x = 0.0;
            let mut y = 0.0;
            let y0 = ((h as f32) / (height as f32)) * 2.5 - 1.12;
            let mut iteration = 0;
            while x * x + y * y <= 4.0 && iteration < MAX_ITERATION {
                let tmp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = tmp;
                iteration += 1;
            }
            img.put_pixel(w, h, set_rgb(iteration));
        }
    }
    img
}
