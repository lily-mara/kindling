use axum::async_trait;
use eyre::Result;
use skia_safe::{utils::text_utils::Align, Canvas, Color4f, Font, FontMgr, Paint, Rect};

use crate::Handler;

pub(crate) struct ErrorHandler {
    pub error: eyre::Report,
}

#[async_trait]
impl Handler for ErrorHandler {
    type Data = ();

    async fn load(&self) -> Result<()> {
        Ok(())
    }

    fn draw(&self, canvas: &Canvas, _data: ()) -> Result<()> {
        let font_mgr = FontMgr::new();
        let typeface = font_mgr
            .new_from_data(include_bytes!("../../media/OpenSansEmoji.ttf"), None)
            .unwrap();

        let big_font = Font::new(&typeface, 36.0);
        let small_font: skia_safe::Handle<_> = Font::new(&typeface, 20.0);

        let black_paint = Paint::new(Color4f::new(0.0, 0.0, 0.0, 1.0), None);
        let light_grey_paint = Paint::new(Color4f::new(0.8, 0.8, 0.8, 1.0), None);

        let image_info = canvas.image_info();

        canvas.draw_rect(
            Rect::new(
                0.0,
                image_info.height() as f32 - 30.0,
                image_info.width() as f32,
                image_info.height() as f32,
            ),
            &light_grey_paint,
        );
        canvas.draw_line(
            (0.0, image_info.height() as f32 - 30.0),
            (image_info.width() as f32, image_info.height() as f32 - 30.0),
            &black_paint,
        );
        canvas.draw_str_align(
            format!("Kindling built {}", crate::BUILD_DATE),
            (
                image_info.width() as f32 / 2.0,
                image_info.height() as f32 - 8.0,
            ),
            &small_font,
            &black_paint,
            Align::Center,
        );

        canvas.draw_rect(
            Rect::new(0.0, 0.0, image_info.width() as f32, 55.0),
            &light_grey_paint,
        );
        canvas.draw_line((0.0, 55.0), (image_info.width() as f32, 55.0), &black_paint);

        canvas.draw_str_align(
            "ERROR",
            (image_info.width() / 2, 40),
            &big_font,
            &black_paint,
            Align::Center,
        );
        let mut y = 90.0;

        let error_second_line_x = {
            let (text_width, _) = small_font.measure_str("- ", Some(&black_paint));
            20.0 + text_width
        };

        for e in self.error.chain() {
            let err_str_full = format!("{e}");
            let mut err_str_build = String::from("- ");
            let mut err_str_build_next = String::from("- ");
            let mut x = 20.0;

            for segment in err_str_full.split(" ") {
                err_str_build_next.push(' ');
                err_str_build_next.push_str(segment);

                let (text_width, _) =
                    small_font.measure_str(&err_str_build_next, Some(&black_paint));
                if 20.0 + text_width > image_info.width() as f32 - 20.0 {
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

        Ok(())
    }
}
