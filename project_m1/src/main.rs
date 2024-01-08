use project_m1::WidgetState;
use druid::{AppLauncher, WindowDesc, LocalizedString};
use screenshots::Screen;
use druid::kurbo::{Size};
const WINDOW_TITLE: LocalizedString<WidgetState> = LocalizedString::new("Screenshot");

fn main() {
    /*let mut width = 0;
    let mut height = 0;
    let screens = Screen::all().unwrap();*/

    /*for screen in screens {
        width = screen.display_info.width;
        height = screen.display_info.height;
    }*/
    // describe the main window
    let main_window = WindowDesc::new(WidgetState::build_root_widget())
        .title(WINDOW_TITLE)
        .transparent(true)
        .set_window_state(druid::WindowState::Maximized)
        //.window_size(Size::new(width as f64, height as f64));
        .show_titlebar(false);

    // create the initial app state
    let initial_state = WidgetState {
        screenshot: None,   // da testare
        start_point: None,
        end_point: None,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
