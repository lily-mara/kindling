use eyre::{bail, eyre, Result};
use serde::Deserialize;
use skia_safe::{
    utils::text_utils::Align, Bitmap, Canvas, Color4f, Font, FontMgr, ImageInfo, Paint, Point, Rect,
};

use crate::{Handler, ImageParams};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub enum RenderTarget {
    #[serde(rename = "kindle")]
    Kindle,

    #[serde(rename = "browser")]
    Browser,
}

fn render_ctx(params: ImageParams, closure: impl FnOnce(&Canvas) -> Result<()>) -> Result<Vec<u8>> {
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

    closure(&canvas)?;

    let image = bitmap.as_image();

    let image_data = image
        .encode(None, skia_safe::EncodedImageFormat::PNG, None)
        .ok_or(eyre!("failed to encode skia image"))?;

    Ok(image_data.as_bytes().into())
}

fn draw_error(canvas: &Canvas, params: ImageParams, error: eyre::Report) {
    let font_mgr = FontMgr::new();
    let typeface = font_mgr
        .new_from_data(include_bytes!("../media/OpenSansEmoji.ttf"), None)
        .unwrap();

    let big_font = Font::new(&typeface, 36.0);
    let small_font: skia_safe::Handle<_> = Font::new(&typeface, 20.0);

    let black_paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
    let light_grey_paint = Paint::new(Color4f::new(0.8, 0.8, 0.8, 1.0), None);

    canvas.draw_rect(
        Rect::new(
            0.0,
            params.height as f32 - 30.0,
            params.width as f32,
            params.height as f32,
        ),
        &light_grey_paint,
    );
    canvas.draw_line(
        (0.0, params.height as f32 - 30.0),
        (params.width as f32, params.height as f32 - 30.0),
        &black_paint,
    );
    canvas.draw_str_align(
        format!("Kindling built {}", crate::BUILD_DATE),
        (params.width as f32 / 2.0, params.height as f32 - 8.0),
        &small_font,
        &black_paint,
        Align::Center,
    );

    canvas.draw_rect(
        Rect::new(0.0, 0.0, params.width as f32, 55.0),
        &light_grey_paint,
    );
    canvas.draw_line((0.0, 55.0), (params.width as f32, 55.0), &black_paint);

    canvas.draw_str_align(
        "ERROR",
        (params.width / 2, 40),
        &big_font,
        &black_paint,
        Align::Center,
    );
    let mut y = 90.0;

    let error_second_line_x = {
        let (text_width, _) = small_font.measure_str("- ", Some(&black_paint));
        20.0 + text_width
    };

    for e in error.chain() {
        let err_str_full = format!("{e}");
        let mut err_str_build = String::from("- ");
        let mut err_str_build_next = String::from("- ");
        let mut x = 20.0;

        for segment in err_str_full.split(" ") {
            err_str_build_next.push(' ');
            err_str_build_next.push_str(segment);

            let (text_width, _) = small_font.measure_str(&err_str_build_next, Some(&black_paint));
            if 20.0 + text_width > params.width as f32 - 20.0 {
                canvas.draw_str(&err_str_build, (x, y), &small_font, &black_paint);
                err_str_build.clear();
                err_str_build_next.clear();
                x = error_second_line_x;
                y += 20.0;
            } else {
                err_str_build.push(' ');
                err_str_build.push_str(segment);
            }
        }
        canvas.draw_str(&err_str_build, (x, y), &small_font, &black_paint);
        y += 20.0;
    }
}

pub fn png_handler(params: ImageParams, handler: &impl Handler) -> Result<Vec<u8>> {
    let image_data = render_ctx(params, |canvas| {
        handler.handle(canvas, params)?;

        Ok(())
    })?;

    Ok(image_data)
}

pub fn error_png(params: ImageParams, error: eyre::Report) -> Result<Vec<u8>> {
    let data = render_ctx(params, move |canvas| {
        draw_error(canvas, params, error);
        Ok(())
    })?;

    Ok(data)
}
