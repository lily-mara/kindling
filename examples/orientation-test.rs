use axum::{async_trait, response::Html, routing::get, Router};
use eyre::Result;
use kindling::{ApplicationBuilder, Handler, Orientation};
use skia_safe::{utils::text_utils::Align, Canvas, Color4f, Font, FontMgr, Paint, Rect};
use tracing_subscriber::EnvFilter;

struct LandscapeHandler;
struct PortraitHandler;

fn draw(canvas: &Canvas) {
    let font_mgr = FontMgr::new();
    let typeface = font_mgr
        .new_from_data(include_bytes!("OpenSansEmoji.ttf"), None)
        .unwrap();

    let font = Font::new(typeface, 10.0);
    let mut paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);

    let mut white_paint = Paint::new(Color4f::new(1.0, 1.0, 1.0, 1.0), None);

    canvas.draw_rect(
        Rect::from_xywh(
            0.0,
            0.0,
            canvas.image_info().width() as f32,
            canvas.image_info().height() as f32,
        ),
        &paint,
    );

    canvas.draw_rect(
        Rect::from_xywh(
            1.0,
            1.0,
            canvas.image_info().width() as f32 - 2.0,
            canvas.image_info().height() as f32 - 2.0,
        ),
        &white_paint,
    );

    canvas.draw_str("Top Left!", (0, 10), &font, &paint);
    canvas.draw_str(
        "Bottom Left!",
        (0, canvas.image_info().height() - 1),
        &font,
        &paint,
    );

    canvas.draw_str_align(
        "Top Right!",
        (canvas.image_info().width(), 10),
        &font,
        &paint,
        Align::Right,
    );
    canvas.draw_str_align(
        "Bottom Right!",
        (
            canvas.image_info().width(),
            canvas.image_info().height() - 1,
        ),
        &font,
        &paint,
        Align::Right,
    );
}

#[async_trait]
impl Handler for PortraitHandler {
    type Data = ();

    fn orientation() -> Orientation {
        Orientation::Portrait
    }

    async fn load(&self) -> Result<()> {
        Ok(())
    }

    fn draw(&self, canvas: &skia_safe::Canvas, _data: ()) -> eyre::Result<()> {
        draw(canvas);
        Ok(())
    }
}

#[async_trait]
impl Handler for LandscapeHandler {
    type Data = ();

    fn orientation() -> Orientation {
        Orientation::Landscape
    }

    async fn load(&self) -> Result<()> {
        Ok(())
    }

    fn draw(&self, canvas: &skia_safe::Canvas, _data: ()) -> eyre::Result<()> {
        draw(canvas);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = ApplicationBuilder::new(
        Router::new().route(
            "/test.html",
            get(|| async { Html(include_str!("orientation-test.html")) }),
        ),
        "http://localhost:3000",
    )
    .add_handler("/landscape.png", LandscapeHandler {})
    .add_handler("/portrait.png", PortraitHandler {})
    .attach();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
