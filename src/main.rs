#![windows_subsystem = "windows"]
mod core;

use druid::{AppLauncher, WindowDesc};
use dnd5e::windows::{Login, Room, Message};


fn main() {
    let default_login = Login::default();

    let main_window = WindowDesc::new(Login::builder)
        .title("SFRPG - D&D - Login")
        .window_size((250., 300.))
        .resizable(false);

    AppLauncher::with_window(main_window).launch(default_login).expect("launch failed");
}
