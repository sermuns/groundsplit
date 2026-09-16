use piet_common::{
    Brush, CairoRenderContext, CairoText, Color, FontFamily, RenderContext, Text, TextLayout,
    TextLayoutBuilder, TextStorage,
    kurbo::{Line, Point, Rect, Size, Vec2},
};

use crate::Split;

pub const BLACK_BACKGROUND: u32 = Color::grey8(16).as_rgba_u32();
pub const DARK_BACKGROUND: u32 = Color::grey8(24).as_rgba_u32();
pub const GRAY_BACKGROUND: u32 = Color::grey8(128).as_rgba_u32();
pub const TEXT_COLOR: Color = Color::WHITE;

/// #3063e5
pub const LIGHT_BLUE: u32 = 0x3063e5ff;
/// #15306e
pub const DARK_BLUE: u32 = 0x3063e5ff;

/// #51e076
pub const LIGHT_GREEN: u32 = 0x51e076ff;
/// #14a93b
pub const DARK_GREEN: u32 = 0x14a93bff;

pub const SPLIT_HEIGHT_IN: f64 = 0.5;
pub const TITLE_HEIGHT_IN: f64 = 1.5 * SPLIT_HEIGHT_IN;
pub const TIME_HEIGHT_IN: f64 = 2.0 * SPLIT_HEIGHT_IN;
pub const LINE_THICKNESS_IN: f64 = 0.01;

pub const FONT_FAMILY: FontFamily = FontFamily::SANS_SERIF;
pub const FONT_SIZE: f64 = 25.;
pub const TIME_FONT_SIZE: f64 = FONT_SIZE * 2.0;

pub struct Context<'a> {
    ctx: &'a mut CairoRenderContext<'a>,
    text: CairoText,
}

impl<'a> Context<'a> {
    pub fn new(ctx: &'a mut CairoRenderContext<'a>) -> Self {
        let text = CairoText::new();
        Self { ctx, text }
    }

    pub fn draw(
        &mut self,
        title: impl TextStorage,
        splits: &[Split],
        time: impl TextStorage,
        width_px: usize,
        height_px: usize,
        padding: f64,
        dpi: f64,
    ) {
        let Self { ctx, text } = self;

        let width_px_f64 = width_px as f64;
        let height_px_f64 = height_px as f64;
        let split_height_px = SPLIT_HEIGHT_IN * dpi;
        let line_thickness_px = LINE_THICKNESS_IN * dpi;
        let padding_px = padding * dpi;

        ctx.fill(
            Rect::from_origin_size(
                Point::ZERO,
                Size {
                    width: width_px_f64,
                    height: height_px_f64,
                },
            ),
            &Brush::Solid(BLACK_BACKGROUND),
        );

        let title_text = text
            .new_text_layout(title)
            .text_color(TEXT_COLOR)
            .font(FONT_FAMILY, FONT_SIZE)
            .build()
            .unwrap();
        let title_rect = Rect::from_origin_size(
            Point::ZERO,
            Size {
                width: width_px_f64,
                height: TITLE_HEIGHT_IN * dpi,
            },
        );
        ctx.draw_text(
            &title_text,
            title_rect.center()
                - Vec2 {
                    x: title_text.size().width / 2.,
                    y: title_rect.size().height / 4., // HACK: random ass divide by 4
                },
        );

        for (i, split) in splits.iter().enumerate() {
            let split_rect = Rect::from_origin_size(
                Point {
                    x: 0.,
                    y: title_rect.height() + (split_height_px + line_thickness_px) * i as f64,
                },
                Size {
                    width: width_px_f64,
                    height: split_height_px,
                },
            );
            if i % 2 == 1 {
                ctx.fill(split_rect, &Brush::Solid(DARK_BACKGROUND));
            }
            let name_text = text
                .new_text_layout(split.name.clone()) // TODO: less alloc
                .text_color(TEXT_COLOR)
                .font(FONT_FAMILY, FONT_SIZE)
                .build()
                .unwrap();

            let middle_y_px = split_rect.center().y - name_text.size().height / 2.;

            ctx.draw_text(
                &name_text,
                Point {
                    x: padding_px,
                    y: middle_y_px,
                },
            );

            let split_time_text = text
                .new_text_layout(split.duration_since_start_str())
                .text_color(TEXT_COLOR)
                .font(FONT_FAMILY, FONT_SIZE)
                .build()
                .unwrap();
            ctx.draw_text(
                &split_time_text,
                Point {
                    x: width_px_f64 - split_time_text.size().width - padding_px,
                    y: middle_y_px,
                },
            );

            ctx.stroke(
                Line::new(
                    Point {
                        x: split_rect.min_x(),
                        y: split_rect.max_y(),
                    },
                    Point {
                        x: split_rect.max_x(),
                        y: split_rect.max_y(),
                    },
                ),
                &Brush::Solid(GRAY_BACKGROUND),
                line_thickness_px,
            );
        }

        let time_height_px = TIME_HEIGHT_IN * dpi;
        let time_y_px =
            title_rect.height() + (split_height_px + line_thickness_px) * splits.len() as f64;

        let time_text = text
            .new_text_layout(time)
            .text_color(TEXT_COLOR)
            .font(FONT_FAMILY, TIME_FONT_SIZE)
            .build()
            .unwrap();
        ctx.draw_text(
            &time_text,
            Point {
                x: width_px_f64 - time_text.size().width - padding_px,
                y: time_y_px,
            },
        );

        ctx.finish().unwrap();
    }
}
