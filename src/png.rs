use eyre::{bail, eyre, Result};
use serde::Deserialize;
use skia_safe::{Bitmap, Canvas, Color4f, ImageInfo, Point};

use crate::{Handler, ImageParams};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub enum RenderTarget {
    #[serde(rename = "kindle")]
    Kindle,

    #[serde(rename = "browser")]
    Browser,
}

pub fn png_handler<H, D>(params: ImageParams, handler: &H, data: D) -> Result<Vec<u8>>
where
    H: Handler<Data = D>,
{
    let dimensions = if params.target == RenderTarget::Kindle {
        (params.height, params.width)
    } else {
        (params.width, params.height)
    };

    let mut bitmap = Bitmap::new();
    if !bitmap.set_info(
        &ImageInfo::new(
            dimensions,
            skia_safe::ColorType::Gray8,
            skia_safe::AlphaType::Unknown,
            None,
        ),
        None,
    ) {
        bail!("failed to initialize skia bitmap");
    }
    bitmap.alloc_pixels();

    let canvas =
        Canvas::from_bitmap(&bitmap, None).ok_or(eyre!("failed to construct skia canvas"))?;
    if params.target == RenderTarget::Kindle {
        canvas.rotate(
            90.0,
            Some(Point::new(
                params.height as f32 / 2.0,
                params.height as f32 / 2.0,
            )),
        );
    }

    canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));

    handler.draw(&canvas, params, data)?;

    let image = bitmap.as_image();

    let image_data = image
        .encode(None, skia_safe::EncodedImageFormat::PNG, None)
        .ok_or(eyre!("failed to encode skia image"))?;

    Ok(image_data.as_bytes().into())
}
