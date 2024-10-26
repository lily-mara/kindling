use axum::async_trait;
use eyre::Result;
use skia_safe::Canvas;

mod blackout;
mod error;

pub(crate) use self::blackout::BlackoutHandler;
pub(crate) use self::error::ErrorHandler;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[async_trait]
pub trait Handler {
    type Data;

    async fn load(&self) -> Result<Self::Data>;

    fn draw(&self, canvas: &Canvas, data: Self::Data) -> Result<()>;

    /// Returns the orientation for the image that this handler renders. Used to
    /// determine if a hard-rotated image should be produced for the Kindle.
    fn orientation() -> Orientation {
        Orientation::Landscape
    }
}
