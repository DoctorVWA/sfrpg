#![windows_subsystem = "windows"]
use druid::{AppLauncher, WindowDesc};
use windows::room::Room;

mod core;
mod windows;

fn main() {
    let mut default_game = Room::default();

    let main_window = WindowDesc::new(Room::builder)
        .title("SFRPG")
        .window_size((800., 600.))
        .with_min_size((800., 600.));

    AppLauncher::with_window(main_window).launch(default_game).expect("launch failed");
}

