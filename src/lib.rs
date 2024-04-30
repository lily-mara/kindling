use axum::{
    body::{Body, Bytes},
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use eyre::{Context, Result};
use serde::Deserialize;
use skia_safe::Canvas;
use std::sync::Arc;

use crate::png::RenderTarget;

mod blackout_handler;
mod png;

const BUILD_DATE: &'static str = env!("BUILD_DATE");

pub trait Handler {
    fn handle(&self, canvas: &Canvas, props: ImageParams) -> Result<()>;
}

pub struct ApplicationBuilder {
    router: Router,
}

impl ApplicationBuilder {
    pub fn new(router: Router) -> Self {
        Self { router }
    }

    pub fn add_handler<H>(mut self, path: &str, handler: H) -> Self
    where
        H: 'static + Send + Sync + Handler,
    {
        let handler = Arc::new(handler);
        let inner_path = path.to_owned();

        self.router = self.router.route(
            path,
            get(|params: Option<Query<ImageParams>>| async move {
                let params = params.unwrap_or_default();

                let data = png::png_handler(params.0, &*handler)
                    .wrap_err_with(|| format!("{}::handle", std::any::type_name::<H>()))
                    .wrap_err_with(|| inner_path.clone())
                    .wrap_err_png(params.0)?;

                Ok::<_, ErrorPng>(
                    Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "image/png")
                        .body(Body::from(Bytes::from(data)))
                        .unwrap(),
                )
            }),
        );
        self
    }

    pub fn attach(self) -> Router {
        self.add_handler(
            "/kindling/v0.1/black.png",
            blackout_handler::BlackoutHandler,
        )
        .router
    }
}

struct ErrorPng {
    data: Vec<u8>,
}

trait WrapErrPng<T> {
    fn wrap_err_png(self, params: ImageParams) -> Result<T, ErrorPng>;
}

impl<T> WrapErrPng<T> for eyre::Result<T> {
    fn wrap_err_png(self, params: ImageParams) -> Result<T, ErrorPng> {
        match self {
            Ok(x) => Ok(x),
            Err(error) => Err(ErrorPng {
                data: png::error_png(params, error).unwrap(),
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
pub struct ImageParams {
    #[serde(default = "default_render_target")]
    pub target: RenderTarget,

    #[serde(default = "default_width")]
    pub width: i32,

    #[serde(default = "default_height")]
    pub height: i32,
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
