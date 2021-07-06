use image::Rgba;
use rusttype::{Font, Scale, point};

impl super::Renderer2D {
    pub fn gfx_print(&self, font: &Font, text: &str, xoff: f32, yoff: f32) {
        let font_size = 24.0; //TODO: Don't hardcode this???? Also copy the line below to wherever you start passing this to calculate it properly
        // let scale = (font_size * window_ctx.window().scale_factor() as f32).round();
        let scale = Scale::uniform(font_size);

        let font_colour = (255, 255, 255);

        let v_metrics = font.v_metrics(scale);

        let glyphs: Vec<_> = font.layout(text, scale, point(0f32, v_metrics.ascent)).collect();
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            max_x as u32
        };

        {
            let mut image_lock = self.font_image.lock().unwrap();
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    // Draw the glyph into the image per-pixel by using the draw closure
                    glyph.draw(|x, y, v| {
                        if (x + xoff as u32 + bounding_box.min.x as u32) < image_lock.width() && (y + yoff as u32 + bounding_box.min.y as u32) < image_lock.height() {
                            image_lock.put_pixel(
                                // Offset the position by the glyph bounding box
                                x + xoff as u32 + bounding_box.min.x as u32,
                                y + yoff as u32 + bounding_box.min.y as u32,
                                // Turn the coverage into an alpha value
                                Rgba([font_colour.0, font_colour.1, font_colour.2, (v * 255.0) as u8]),
                            )
                        }
                    });
                }
            }
        }

    }
}
