use axum::Router;
use eyre::bail;
use kindling::{ApplicationBuilder, Handler, ImageParams};
use tracing_subscriber::EnvFilter;

struct ErrorHandler;

impl Handler for ErrorHandler {
    fn handle(&self, _canvas: &skia_safe::Canvas, _props: ImageParams) -> eyre::Result<()> {
        bail!("I always return an error");
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = ApplicationBuilder::new(Router::new())
        .add_handler("/error.png", ErrorHandler)
        .attach();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
