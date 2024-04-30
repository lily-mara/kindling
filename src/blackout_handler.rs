use eyre::Result;
use skia_safe::{Color4f, Paint, Rect};

use crate::Handler;

pub(crate) struct BlackoutHandler;

impl Handler for BlackoutHandler {
    fn handle(&self, canvas: &skia_safe::Canvas, params: crate::ImageParams) -> Result<()> {
        let black_paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);

        canvas.draw_rect(
            Rect::new(0.0, 0.0, params.width as f32, params.height as f32),
            &black_paint,
        );

        Ok(())
    }
}
