use image::{self, RgbImage};

const MAX_ITERATION: i32 = 30;

pub fn set_rgb(iteration: i32) -> image::Rgb<u8> {
    let col = (((iteration as f32) / (MAX_ITERATION as f32)) * 255.0).round() as u8;
    // print!("{}\n", col);
    return image::Rgb([130, 35, col]);
}

pub fn generate_image(width: u32, height: u32) -> image::RgbImage {
    let mut img = RgbImage::new(width, height);
    for w in 0..width {
        let x0 = (((w as f32) / (width as f32)) * 3.5) - 2.5;
        for h in 0..height {
            let mut x = 0.0;
            let mut y = 0.0;
            let y0 = (((h as f32) / (height as f32)) * 2.5) - 1.2;
            let mut iteration = 0;
            // x^2 + y^2 <= 4.0
            while x * x + y * y <= 4.0 && iteration < MAX_ITERATION {
                // x^2 - y^2
                let tmp = x * x - y * y + x0;
                // 2xy
                y = 2.0 * x * y + y0;
                x = tmp;
                iteration += 1;
            }
            img.put_pixel(w, h, set_rgb(iteration));
        }
    }
    img
}
