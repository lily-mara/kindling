use eyre::{bail, eyre, Result};
use serde::Deserialize;
use skia_safe::{image::Image, Bitmap, Canvas, Color4f, ImageInfo, Point};

use crate::{Handler, ImageParams, Orientation};

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
    let mut bitmap = Bitmap::new();
    if !bitmap.set_info(
        &ImageInfo::new(
            (params.width, params.height),
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

    canvas.clear(Color4f::new(1.0, 1.0, 1.0, 1.0));

    handler.draw(&canvas, data)?;

    let image = bitmap.as_image();

    let image =
        if params.target == RenderTarget::Kindle && H::orientation() == Orientation::Landscape {
            rotate_image(image, canvas.image_info())?
        } else {
            image
        };

    let image_data = image
        .encode(None, skia_safe::EncodedImageFormat::PNG, None)
        .ok_or(eyre!("failed to encode skia image"))?;

    Ok(image_data.as_bytes().into())
}

fn rotate_image(original_image: Image, original_info: ImageInfo) -> Result<Image> {
    let mut bitmap = Bitmap::new();
    if !bitmap.set_info(
        &ImageInfo::new(
            (original_info.height(), original_info.width()),
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
    canvas.rotate(
        90.0,
        Some(Point::new(
            original_info.height() as f32 / 2.0,
            original_info.height() as f32 / 2.0,
        )),
    );
    canvas.draw_image(original_image, (0, 0), None);
    Ok(bitmap.as_image())
}
