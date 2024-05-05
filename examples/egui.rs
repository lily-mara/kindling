use std::sync::atomic::{AtomicBool, Ordering};

use axum::{async_trait, Router};
use egui::Visuals;
use eyre::{bail, Result};
use kindling::{ApplicationBuilder, Handler, ImageParams};
use skia_safe::Canvas;
use tracing_subscriber::EnvFilter;

struct EguiHandler {}

#[async_trait]
impl Handler for EguiHandler {
    type Data = ();

    async fn load(&self) -> Result<()> {
        Ok(())
    }

    fn draw(&self, canvas: &Canvas, _props: ImageParams, _data: ()) -> eyre::Result<()> {
        smol_egui_skia::draw_onto_canvas(
            canvas,
            |ctx| {
                ctx.set_visuals(Visuals::light());
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Hello world!");
                });
            },
            None,
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = ApplicationBuilder::new(Router::new(), "http://10.0.0.50:3000")
        .add_handler("/image.png", EguiHandler {})
        .attach();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
