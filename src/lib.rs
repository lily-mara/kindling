use axum::{
    body::{Body, Bytes},
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use chrono::Utc;
use eyre::{Context, Result};
use serde::Deserialize;
use std::{borrow::Cow, sync::Arc};

use crate::png::RenderTarget;

mod handler;
mod png;

pub use crate::handler::{Handler, Orientation};

const BUILD_DATE: &'static str = env!("BUILD_DATE");

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct StopsTemplate<'a> {
    routes: &'a [String],
    base_url: &'a str,
}

pub struct ApplicationBuilder {
    router: Router<Data>,
    state: Data,
}

#[derive(Clone)]
pub struct Data {
    routes: Vec<String>,
    base_url: Cow<'static, str>,
}

#[derive(Deserialize)]
struct InstallQuery {
    route: String,
}

impl ApplicationBuilder {
    pub fn new(router: Router<Data>, base_url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            router,
            state: Data {
                routes: Vec::new(),
                base_url: base_url.into(),
            },
        }
    }

    pub fn add_handler(
        mut self,
        path: &str,
        handler: impl 'static + Send + Sync + Handler,
    ) -> Self {
        let handler = Arc::new(handler);
        let inner_path = path.to_owned();

        self.state.routes.push(inner_path.clone());

        self.router = self.router.route(
            path,
            get(|params: Option<Query<ImageParams>>| async move {
                let params = params.unwrap_or_default();

                match handler_inner(&*handler, params.0).await {
                    Ok(x) => Ok(x),
                    Err(error) => {
                        let error = error.wrap_err(inner_path.clone());
                        Err(ErrorPng {
                            data: png::png_handler(params.0, &handler::ErrorHandler { error }, ())
                                .unwrap(),
                        })
                    }
                }
            }),
        );
        self
    }

    pub fn attach(self) -> Router {
        let state = self.state.clone();

        self.add_handler("/kindling/v0.1/black.png", handler::BlackoutHandler)
            .router
            .route(
                "/kindling/v0.1/refresh-image.sh",
                get(|| async {
                    include_str!("../refresh-image.sh")
                        .replace("SUB_BUILD_TIME", BUILD_DATE)
                        .replace("SUB_FETCH_TIME", &format!("{}", Utc::now()))
                }),
            )
            .route(
                "/kindling/v0.1/install",
                get(
                    |state: State<Data>, query: Query<InstallQuery>| async move {
                        include_str!("../install")
                            .replace("SUB_BASE_URL", &state.base_url)
                            .replace("SUB_ROUTE", &query.route)
                    },
                ),
            )
            .route(
                "/",
                get(|state: State<Data>| async move {
                    Html(
                        StopsTemplate {
                            routes: &state.routes,
                            base_url: state.base_url.as_ref(),
                        }
                        .render()
                        .unwrap(),
                    )
                }),
            )
            .with_state(state)
    }
}

async fn handler_inner<H>(handler: &H, params: ImageParams) -> Result<Response<Body>>
where
    H: 'static + Send + Sync + Handler,
{
    let data = handler
        .load()
        .await
        .wrap_err_with(|| format!("{}::load", std::any::type_name::<H>()))?;

    let image_data = png::png_handler(params, &*handler, data)
        .wrap_err_with(|| format!("{}::handle", std::any::type_name::<H>()))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(Body::from(Bytes::from(image_data)))
        .unwrap())
}

struct ErrorPng {
    data: Vec<u8>,
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
    754
}
fn default_height() -> i32 {
    1058
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
