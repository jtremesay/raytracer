use crate::color::Color;
use crate::math::lerp_color8;
use image::ImageResult;
use image::Rgb;
use image::RgbImage;
use sdl2::pixels::Color as SDL2Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas as SDL2Canvas;
use sdl2::render::RenderTarget;
use sdl2::render::Texture;
use std::iter::repeat;

/**
 * @brief Interface for a canvas, a surface on which we can draw
 *
 * (0, 0) is bottom left and (width - 1, height - 1) is top right
 */
pub trait Canvas {
    /**
     * @brief Draw a pixel on the canvas
     *
     * @param x X coordinate
     * @param y Y coordinate
     * @param color the color of the pixel
     */
    fn draw_pixel(&mut self, x: u32, y: u32, color: Color);

    /**
     * @brief Get the width of the canvas
     *
     * @return the width of the canvas
     */
    fn width(&self) -> u32;

    /**
     * @brief Get the height of the canvas
     *
     * @return the height of the canvas
     */
    fn height(&self) -> u32;
}

/**
 * @brief Implement a canvas with an image buffer
 */
pub struct ImageCanvas {
    /** @brief an image buffer */
    image: RgbImage,
}

impl ImageCanvas {
    /**
     * @brief Create a new image canvas
     *
     * @param width the width of the canvas
     * @param height the height of the canvas
     */
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            image: RgbImage::new(width, height),
        }
    }

    /**
     * @brief Save the canvas into an image file
     *
     * @param path path of the image file
     */
    pub fn save(&self, path: &str) -> ImageResult<()> {
        self.image.save(path)
    }
}

impl Canvas for ImageCanvas {
    fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        // Convert the color to R8G8B8
        let color = Rgb([
            lerp_color8(color.r),
            lerp_color8(color.g),
            lerp_color8(color.b),
        ]);

        // flip the image, the library use y=0 for top when we use it
        // for the bottom
        self.image.put_pixel(x, self.image.height() - 1 - y, color)
    }
    fn width(&self) -> u32 {
        self.image.width()
    }

    fn height(&self) -> u32 {
        self.image.height()
    }
}

pub struct SDLCanvas<'a, T: RenderTarget> {
    width: u32,
    height: u32,
    canvas: &'a mut SDL2Canvas<T>,
}

impl<'a, T: RenderTarget> SDLCanvas<'a, T> {
    /**
     * @brief Create a new image canvas
     *
     * @param width the width of the canvas
     * @param height the height of the canvas
     */
    pub fn new(width: u32, height: u32, canvas: &'a mut SDL2Canvas<T>) -> Self {
        Self {
            width,
            height,
            canvas,
        }
    }
}

impl<'a, T: RenderTarget> Canvas for SDLCanvas<'a, T> {
    fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        // Convert the color to R8G8B8
        let color = SDL2Color::RGB(
            lerp_color8(color.r),
            lerp_color8(color.g),
            lerp_color8(color.b),
        );

        // flip the image, the library use y=0 for top when we use it
        // for the bottom
        self.canvas.set_draw_color(color);
        self.canvas
            .draw_rect(Rect::new(
                x as i32,
                self.height() as i32 - 1 - y as i32,
                1,
                1,
            ))
            .unwrap();
        //self.image.put_pixel(x, self.image.height() - 1 - y, color)
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

pub struct FrameBufferCanvas {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl FrameBufferCanvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
            pixels: repeat(Color::MAGENTA)
                .take((width * height) as usize)
                .collect(),
        }
    }
}

impl FrameBufferCanvas {
    pub fn copy_to_texture(&self, texture: &mut Texture) {
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let offset_in = (y * self.width + x) as usize;
                        let offset_out = (self.height - 1 - y) as usize * pitch + x as usize * 3;

                        let color = self.pixels[offset_in];
                        buffer[offset_out] = lerp_color8(color.r);
                        buffer[offset_out + 1] = lerp_color8(color.g);
                        buffer[offset_out + 2] = lerp_color8(color.b);
                    }
                }
            })
            .unwrap();
    }
}

impl Canvas for FrameBufferCanvas {
    fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels[(y * self.width + x) as usize] = color;
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}
