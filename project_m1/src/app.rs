use image::{DynamicImage, GenericImageView};
use druid::kurbo::{Rect, Size};
use druid::{Color, commands, Data, ImageBuf, Lens, Point, RenderContext, Selector, Widget, WidgetExt};
use druid::piet::ImageFormat;
use druid::widget::prelude::*;
use druid::widget::{Controller, Flex, Painter, ViewSwitcher};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

#[derive(Clone, Data, Lens)]
pub struct WidgetState {
    pub start_point: Option<Point>,
    pub end_point: Option<Point>,
}

impl WidgetState {
    pub fn build_root_widget() -> impl Widget<WidgetState> {
        Flex::column()
            .with_child(
                Painter::new(|ctx, data: &WidgetState, _env| {
                    // Disegna il rettangolo
                    if let (Some(start), Some(end)) = (data.start_point, data.end_point) {
                        let rect = Rect::from_points(start, end);
                        ctx.fill(rect, &Color::rgba8(50, 128, 128, 128));
                        ctx.stroke(rect, &Color::GRAY, 1.0);
                    }
                })
                    .fix_height(300.0)
                    .expand_width(),
            )
            .controller(DrawRectController)
    }
}

struct DrawRectController;
const DRAW_RECT: Selector<()> = Selector::new("draw-rect");

impl < W: Widget<WidgetState> > Controller<WidgetState, W> for DrawRectController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut WidgetState, env: &Env) {
        match event {
            Event::MouseDown(mouse) => {
                data.start_point = Some(mouse.pos);
                data.end_point = None;
                ctx.request_paint();
            }
            Event::MouseMove(mouse) => {
                if mouse.buttons.has_left() {
                    data.end_point = Some(mouse.pos);
                    ctx.request_paint();
                }
            }
            Event::MouseUp(mouse) => {
                if mouse.button.is_left() {
                    data.end_point = Some(mouse.pos);
                    ctx.request_paint();
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}
