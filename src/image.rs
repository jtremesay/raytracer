use crate::canvas::Canvas;
use crate::math::lerp_color8;
use image::Rgb;
use image::RgbImage;
use std::path::Path;

pub fn save_canvas_to_file(canvas: &dyn Canvas, path: &Path) {
    let (width, height) = (canvas.width(), canvas.height());
    let mut image = RgbImage::new(width, height);
    for (x, y, p) in image.enumerate_pixels_mut() {
        let in_color = canvas.get_pixel(x, height - y - 1);
        *p = Rgb([
            lerp_color8(in_color.r),
            lerp_color8(in_color.g),
            lerp_color8(in_color.b),
        ]);
    }

    image.save(path).unwrap();
}
