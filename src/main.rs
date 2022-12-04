mod mandelbrotlib;

fn main() {
    let (fname,width,height) = ("square.png",1000,800);
    let img = mandelbrotlib::lib::generate_image(width, height);
    img.save_with_format(fname, image::ImageFormat::Png).unwrap();
}
