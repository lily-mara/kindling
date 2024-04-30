use axum::Router;
use eyre::bail;
use tracing_subscriber::EnvFilter;
use transit_kindle::{ApplicationBuilder, Handler};

struct ErrorHandler;

impl Handler for ErrorHandler {
    fn handle(
        &self,
        _canvas: &skia_safe::Canvas,
        _props: transit_kindle::ImageParams,
    ) -> eyre::Result<()> {
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
