use crate::color::Color;
use crate::math::lerp;
use image::ImageResult;
use image::Rgb;
use image::RgbImage;

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
            lerp(color.r, 0.0, 1.0, 0.0, 255.0) as u8,
            lerp(color.g, 0.0, 1.0, 0.0, 255.0) as u8,
            lerp(color.b, 0.0, 1.0, 0.0, 255.0) as u8,
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
