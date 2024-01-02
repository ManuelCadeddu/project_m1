use druid::kurbo::{Rect};
use druid::{Color, Data, Lens, Point, RenderContext, Widget, WidgetExt,Selector};
use druid::widget::prelude::*;
use druid::widget::{Controller, Flex, Painter};
use screenshots::Screen;
use std::time::Instant;

#[derive(Clone, Data, Lens)]
pub struct WidgetState {
    pub start_point: Option<Point>,
    // Punto di inizio del rettangolo ed è un f64
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
                        ctx.fill(rect, &Color::rgba8(255, 255, 255, 70));
                        ctx.stroke(rect, &Color::TRANSPARENT, 1.0);
                    }
                })
                    .fix_height(300.0)
                    .expand_width(),
            )
            .controller(DrawRectController)
    }
}

struct DrawRectController;
impl<W: Widget<WidgetState>> Controller<WidgetState, W> for DrawRectController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut WidgetState, env: &Env) {
        match event {
            Event::MouseDown(mouse) => {
                data.start_point = Some(mouse.pos);
                data.end_point = None;
                ctx.request_paint();
                println!("Mouse down: {:?}", mouse.pos);
            }
            Event::MouseMove(mouse) => {
                if mouse.buttons.has_left(){
                    data.end_point = Some(mouse.pos);
                    ctx.request_paint();
                    println!("Mouse move: {:?}", mouse.pos);
                }
            }
            Event::MouseUp(mouse) => {
                if mouse.button.is_left() {
                    data.end_point = Some(mouse.pos);
                    //ctx.submit_command(druid::commands::CLOSE_WINDOW); non serve qua mi chiude la finestra
                    capture_screenshot_area(data.start_point, data.end_point);
                    data.start_point = None; // Reimposta i punti su None
                    data.end_point = None;
                    ctx.request_paint();
                    println!("Mouse up: {:?}", mouse.pos);
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

fn capture_screenshot_area(start: Option<Point>, end: Option<Point>) {
    let start_time = Instant::now();
    let screens = Screen::all().unwrap();

    for screen in screens {
        let mut image = screen.capture().unwrap();
        if let (Some(start), Some(end)) = (start,end) {
            let (start_x, start_y) = (start.x as i32, start.y as i32);
            let (end_x, end_y) = (end.x as u32, end.y as u32);
            println!("start_x: {}, start_y: {}, end_x: {}, end_y: {}", start_x, start_y, end_x, end_y);
            image = screen.capture_area(start_x, start_y, end_x, end_y).unwrap();
            image.save(format!("target/{}-1.png", screen.display_info.id)).unwrap();
        }
    }

    println!("Screenshots catturati e salvati in {} secondi", start_time.elapsed().as_secs_f64());
}

