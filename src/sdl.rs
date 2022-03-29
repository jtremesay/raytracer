use crate::canvas::Canvas;
use crate::math::lerp_color8;
use sdl2::render::Texture;

pub fn copy_canvas_to_texture(canvas: &dyn Canvas, texture: &mut Texture) {
    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let (width, height) = (canvas.width(), canvas.height());
            for y in 0..height {
                for x in 0..width {
                    let color = canvas.get_pixel(x, y);
                    let offset_out = (height - 1 - y) as usize * pitch + x as usize * 3;

                    buffer[offset_out] = lerp_color8(color.r);
                    buffer[offset_out + 1] = lerp_color8(color.g);
                    buffer[offset_out + 2] = lerp_color8(color.b);
                }
            }
        })
        .unwrap();
}
