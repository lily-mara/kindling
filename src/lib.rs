use axum::{routing::get, Router};

mod png;

const BUILD_DATE: &'static str = env!("BUILD_DATE");

pub struct ApplicationBuilder {}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn attach(self, router: Router) -> Router {
        router.route("/kindling/v0.1/black.png", get(handle_black_png))
    }
}

use axum::{
    body::{Body, Bytes},
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use eyre::Context;
use serde::Deserialize;

use crate::png::RenderTarget;

struct ErrorPng {
    data: Vec<u8>,
}

trait WrapErrPng<T> {
    fn wrap_err_png(self, route: &str, params: ImageParams) -> Result<T, ErrorPng>;
}

impl<T> WrapErrPng<T> for eyre::Result<T> {
    fn wrap_err_png(self, route: &str, params: ImageParams) -> Result<T, ErrorPng> {
        match self {
            Ok(x) => Ok(x),
            Err(error) => Err(ErrorPng {
                data: png::error_png(params, route, error).unwrap(),
            }),
        }
    }
}

impl IntoResponse for ErrorPng {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "image/png")
            .body(Body::from(Bytes::from(self.data)))
            .unwrap()
            .into_response()
    }
}

fn default_render_target() -> RenderTarget {
    RenderTarget::Browser
}
fn default_width() -> i32 {
    1058
}
fn default_height() -> i32 {
    754
}

#[derive(Deserialize, Clone, Copy)]
struct ImageParams {
    #[serde(default = "default_render_target")]
    target: RenderTarget,

    #[serde(default = "default_width")]
    width: i32,

    #[serde(default = "default_height")]
    height: i32,
}

impl Default for ImageParams {
    fn default() -> Self {
        Self {
            target: default_render_target(),
            width: default_width(),
            height: default_height(),
        }
    }
}

async fn handle_black_png(params: Option<Query<ImageParams>>) -> Result<Response<Body>, ErrorPng> {
    let params = params.unwrap_or_default();

    let data = png::black_png(params.0)
        .wrap_err("render black box")
        .wrap_err_png("/kindling/v0.1/black.png", params.0)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(Body::from(Bytes::from(data)))
        .unwrap())
}
