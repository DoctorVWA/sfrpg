#![windows_subsystem = "windows"]
mod core;

use druid::{AppLauncher, WindowDesc};
use dnd5e::windows::Room;


fn main() {
    let mut default_game = Room::default();

    let main_window = WindowDesc::new(Room::builder)
        .title("SFRPG")
        .window_size((800., 600.))
        .with_min_size((800., 600.));

    default_game.messages = (0..=20).map(|n| n.to_string()).collect();

    AppLauncher::with_window(main_window).launch(default_game).expect("launch failed");
}
