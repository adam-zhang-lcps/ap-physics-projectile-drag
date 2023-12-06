mod graph;
mod gui;
mod physics;

use gui::*;
use iced::{Application, Settings, window};

fn main() {
    Gui::run(Settings {
        window: window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        ..Settings::default()
    })
    .unwrap();
}
