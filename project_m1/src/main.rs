use project_m1::WidgetState;
use druid::{AppLauncher, WindowDesc, LocalizedString};
const WINDOW_TITLE: LocalizedString<WidgetState> = LocalizedString::new("Screenshot");

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(WidgetState::build_root_widget())
        .title(WINDOW_TITLE)
        .transparent(true)
        .set_window_state(druid::WindowState::Maximized)
        .show_titlebar(false);

    // create the initial app state
    let initial_state = WidgetState {
        start_point: None,
        end_point: None,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}