mod graph;
mod gui;
mod physics;

use gui::*;
use iced::{window, Application, Settings, Size};

fn main() {
    Gui::run(Settings {
        window: window::Settings {
            size: Size {
                width: 800.0,
                height: 600.0,
            },
            ..Default::default()
        },
        ..Settings::default()
    })
    .unwrap();
}
