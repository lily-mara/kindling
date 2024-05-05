use std::sync::atomic::{AtomicBool, Ordering};

use axum::{async_trait, Router};
use eyre::{bail, Result};
use kindling::{ApplicationBuilder, Handler, ImageParams};
use tracing_subscriber::EnvFilter;

struct ErrorHandler {
    can_load: AtomicBool,
}

#[async_trait]
impl Handler for ErrorHandler {
    type Data = ();

    async fn load(&self) -> Result<()> {
        if self.can_load.swap(true, Ordering::SeqCst) {
            return Ok(());
        }

        bail!("I don't load the first time!")
    }

    fn draw(
        &self,
        _canvas: &skia_safe::Canvas,
        _props: ImageParams,
        _data: (),
    ) -> eyre::Result<()> {
        bail!("I always return an error");
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = ApplicationBuilder::new(Router::new(), "http://10.0.0.50:3000")
        .add_handler(
            "/error.png",
            ErrorHandler {
                can_load: AtomicBool::new(false),
            },
        )
        .attach();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
