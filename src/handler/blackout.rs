use axum::async_trait;
use eyre::Result;
use skia_safe::{Color4f, Paint, Rect};

use crate::Handler;

pub(crate) struct BlackoutHandler;

#[async_trait]
impl Handler for BlackoutHandler {
    type Data = ();

    async fn load(&self) -> Result<()> {
        Ok(())
    }

    fn draw(
        &self,
        canvas: &skia_safe::Canvas,
        params: crate::ImageParams,
        _data: (),
    ) -> Result<()> {
        let black_paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);

        canvas.draw_rect(
            Rect::new(0.0, 0.0, params.width as f32, params.height as f32),
            &black_paint,
        );

        Ok(())
    }
}
