use piet_common::{
    Brush, CairoRenderContext, CairoText, Color, FontFamily, RenderContext, Text, TextAlignment,
    TextLayout, TextLayoutBuilder, TextStorage,
    kurbo::{Point, Rect, Size, Vec2},
};

use crate::Split;

/// #101010
pub const BLACK_BACKGROUND: u32 = 0x101010ff;
/// #171717
pub const DARK_BACKGROUND: u32 = 0x171717ff;
/// #1d1d1d
pub const GRAY_BACKGROUND: u32 = 0x1d1d1dff;

/// #3063e5
pub const LIGHT_BLUE: u32 = 0x3063e5ff;
/// #15306e
pub const DARK_BLUE: u32 = 0x3063e5ff;

/// #51e076
pub const LIGHT_GREEN: u32 = 0x51e076ff;
/// #14a93b
pub const DARK_GREEN: u32 = 0x14a93bff;

pub const TEXT_COLOR: Color = Color::WHITE;

pub const BLOCK_HEIGHT_IN: f64 = 0.5;
pub const TITLE_HEIGHT_IN: f64 = 1.5 * BLOCK_HEIGHT_IN;
pub const LINE_THICKNESS_IN: f64 = 0.1;

pub const FONT_FAMILY: FontFamily = FontFamily::SANS_SERIF;

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
        width_px: usize,
        height_px: usize,
        dpi: f64,
    ) {
        let Self { ctx, text } = self;

        let width_px_f64 = width_px as f64;
        let height_px_f64 = height_px as f64;
        let center = Point::new(width_px_f64 / 2., height_px_f64 / 2.);

        ctx.fill(
            Rect::from_origin_size(
                Point::ZERO,
                Size {
                    width: width_px_f64,
                    height: height_px_f64,
                },
            ),
            &Brush::Solid(DARK_BACKGROUND),
        );

        let title_text = text
            .new_text_layout(title)
            .text_color(TEXT_COLOR)
            .font(FONT_FAMILY, 30.)
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
                    y: 0.,
                },
        );

        for (
            i,
            Split {
                name,
                ms_since_start,
            },
        ) in splits.iter().enumerate()
        {
            // TODO: less alloc
            let name_text = text
                .new_text_layout(name.to_owned())
                .text_color(TEXT_COLOR)
                .font(FONT_FAMILY, 30.)
                .build()
                .unwrap();
            ctx.draw_text(
                &name_text,
                Point {
                    x: 0.,
                    y: title_rect.height() + BLOCK_HEIGHT_IN * dpi * (i as f64 + 0.5),
                },
            );
        }

        ctx.finish().unwrap();
    }
}
