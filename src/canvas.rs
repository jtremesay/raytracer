use crate::color::Color;
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
    fn get_pixel(&self, x: u32, y: u32) -> Color;

    /**
     * @brief Draw a pixel on the canvas
     *
     * @param x X coordinate
     * @param y Y coordinate
     * @param color the color of the pixel
     */
    fn set_pixel(&mut self, x: u32, y: u32, color: Color);

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

impl FrameBufferCanvas {}

impl Canvas for FrameBufferCanvas {
    fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.pixels[(y * self.width + x) as usize]
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels[(y * self.width + x) as usize] = color;
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}
