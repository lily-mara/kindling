use axum::async_trait;
use eyre::Result;
use skia_safe::Canvas;

use crate::ImageParams;

mod blackout;
mod error;

pub(crate) use self::blackout::BlackoutHandler;
pub(crate) use self::error::ErrorHandler;

#[async_trait]
pub trait Handler {
    type Data;

    async fn load(&self) -> Result<Self::Data>;

    fn draw(&self, canvas: &Canvas, params: ImageParams, data: Self::Data) -> Result<()>;
}
