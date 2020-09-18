#![windows_subsystem = "windows"]
mod core;

use druid::{AppLauncher, WindowDesc};
use dnd5e::windows::{Room, Message};


fn main() {
    let mut default_game = Room::default();

    let main_window = WindowDesc::new(Room::builder)
        .title("SFRPG")
        .window_size((800., 600.))
        .with_min_size((800., 600.));

    default_game.messages = (0..=20).map(|n| Message {
        author: "Joãozin".to_string(),
        content: n.to_string(),
        date: "10/09/2020 23:23".to_string()
    }).collect();

    AppLauncher::with_window(main_window).launch(default_game).expect("launch failed");
}
