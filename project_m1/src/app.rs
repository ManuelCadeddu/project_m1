use druid::kurbo::{Rect};
use druid::{Color, Data, Lens, Point, RenderContext, Widget, WidgetExt};
use druid::widget::prelude::*;
use druid::widget::{Controller, Flex, Painter,Button,Container,SizedBox};
use screenshots::Screen;
use std::time::Instant;
use image::{RgbaImage, DynamicImage, Rgba};

#[derive(Clone, Data, Lens)]
pub struct WidgetState {
    pub screenshot: Option<RgbaImage>, // da testare
    pub start_point: Option<Point>,
    // Punto di inizio del rettangolo ed è un f64
    pub end_point: Option<Point>,
}

impl WidgetState {
    pub fn build_root_widget(&self) -> impl Widget<WidgetState> {

        let close_window_button = Button::new("Close Window")
            .on_click(|ctx, data: &mut WidgetState, _env| {
                ctx.submit_command(druid::commands::CLOSE_WINDOW);
            });

        // Da testare
        let screenshot_button = Button::new("Screenshot")
            .on_click(|ctx, data: &mut WidgetState, _env| {
                capture_screenshot(data);
            });

        // Da testare
        let background = Container::new(SizedBox::empty().width(200.0).height(200.0))
            .background(self.screenshot.unwrap());

        Flex::row()
            .with_child(close_window_button)
            .with_child(screenshot_button)  // da testare
            .with_child(background);    //da testare
            /*.with_child(
                Painter::new(|ctx, data: &WidgetState, _env| {
                    // Disegna il rettangolo
                    if let (Some(start), Some(end)) = (data.start_point, data.end_point) {
                        let rect = Rect::from_points(start, end);
                        ctx.fill(rect, &Color::rgba8(255, 255, 255, 70));
                        ctx.stroke(rect, &Color::TRANSPARENT, 1.0);
                    }
                })
            )
           .controller(DrawRectController)*/
    }
}

struct DrawRectController;
impl<W: Widget<WidgetState>> Controller<WidgetState, W> for DrawRectController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut WidgetState, env: &Env) {
        match event {
            Event::MouseDown(mouse) => {
                //data.start_point = Some(mouse.pos);
                //data.end_point = None;

                // mod
                data.start_point = Some(ctx.to_screen(mouse.pos));

                ctx.request_paint();
                println!("Mouse down: {:?}", mouse.pos);
                println!("strat_point: {:?}", data.start_point);
            }
            Event::MouseMove(mouse) => {
                if mouse.buttons.has_left(){
                    //data.end_point = Some(mouse.pos);
                    data.end_point = Some(ctx.to_screen(mouse.pos));
                    ctx.request_paint();
                }
            }
            Event::MouseUp(mouse) => {
                if mouse.button.is_left() {
                    data.end_point = Some(ctx.to_screen(mouse.pos));


                    // metto come start.x il valore più piccolo tra star.x e end.x (stessa cosa per y)
                    data.start_point.unwrap().x = if data.start_point.unwrap().x <= data.end_point.unwrap().x { data.start_point.unwrap().x } else { data.end_point.unwrap().x };
                    data.start_point.unwrap().y = if data.start_point.unwrap().y <= data.end_point.unwrap().y { data.start_point.unwrap().y } else { data.end_point.unwrap().y };

                    capture_screenshot_area(data.start_point, data.end_point);
                    data.start_point = None; // Reimposta i punti su None
                    data.end_point = None;
                    ctx.request_paint();
                    println!("Mouse up: {:?}", mouse.pos);
                    println!("end_point: {:?}", data.end_point);
                    println!("start_global: {:?}", data.start_point);
                }
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}


// Da testare
fn capture_screenshot(data: &mut WidgetState) {

    let start_time = Instant::now();
    let screens = Screen::all().unwrap();

    for screen in screens {
        let image_buffer = screen.capture().unwrap();
        image_buffer.save(format!("target/full{}-1.png", screen.display_info.id)).unwrap();
        // Convert the image buffer to RgbaImage
        data.screenshot = RgbaImage::from_raw(image_buffer.width(), image_buffer.height(), image_buffer.into_raw()).unwrap();   //da testare

    }

    // Convert the image buffer to RgbaImage
    //data.screenshot = RgbaImage::from_raw(image_buffer.width(), image_buffer.height(), image_buffer.into_raw());   //da testare

    println!("Screenshots catturati e salvati in {} secondi", start_time.elapsed().as_secs_f64());
}

/*fn capture_screenshot_area(start: Option<Point>, end: Option<Point>) {
    let start_time = Instant::now();
    let screens = Screen::all().unwrap();

    for screen in screens {

        if let (Some(start), Some(end)) = (start, end) {
            let (start_x, start_y) = (start.x as i32, start.y as i32);
            let (end_x, end_y) = (end.x as u32, end.y as u32);

            let width = end_x - start_x as u32;
            let height = end_y - start_y as u32;

            println!("start_x: {}, start_y: {}, width: {}, height: {}", start_x, start_y, width, height);

            let mut image = screen.capture_area(start_x, start_y, width, height).unwrap();
            image.save(format!("target/{}-1.png", screen.display_info.id)).unwrap();
        }
    }

    println!("Screenshots catturati e salvati in {} secondi", start_time.elapsed().as_secs_f64());
}*/

fn capture_screenshot_area(start: Option<Point>, end: Option<Point>) {
    let start_time = Instant::now();
    let screens = Screen::all().unwrap();

    for screen in screens {
        if let (Some(start), Some(end)) = (start.as_ref(), end.as_ref()) {
            let image_buffer = screen.capture().unwrap();

            // Ensure start and end values are unwrapped before using them
            let start_x = start.x as u32;
            let start_y = start.y as u32;
            let end_x = end.x as u32;
            let end_y = end.y as u32;

            // Check for zero width and height
            if start_x >= end_x || start_y >= end_y {
                eprintln!("Error: Zero width or height not allowed");
                continue; // Skip to the next iteration
            }

            // Convert the image buffer to RgbaImage
            let mut image = RgbaImage::from_raw(image_buffer.width(), image_buffer.height(), image_buffer.into_raw()).unwrap();

            // Perform cropping
            let cropped_image = image::imageops::crop(&mut image, start_x, start_y, end_x - start_x, end_y - start_y).to_image();

            // Save cropped image
            if let Err(err) = cropped_image.save(format!("target/{}-1.png", screen.display_info.id)) {
                eprintln!("Error saving cropped image: {}", err);
                // Handle the error as needed
            }
        }
    }

    println!("Screenshots catturati e salvati in {} secondi", start_time.elapsed().as_secs_f64());
}

