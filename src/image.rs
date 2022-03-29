use crate::canvas::Canvas;
use crate::math::lerp_color8;
use image::Rgb;
use image::RgbImage;
use std::path::Path;

pub fn save_canvas_to_file(canvas: &dyn Canvas, path: &Path) {
    let (width, height) = (canvas.width(), canvas.height());

    let mut image = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let in_color = canvas.get_pixel(x, y);
            let out_color = Rgb([
                lerp_color8(in_color.r),
                lerp_color8(in_color.g),
                lerp_color8(in_color.b),
            ]);
            image.put_pixel(x, height - 1 - y, out_color);
        }
    }

    image.save(path).unwrap();
}
