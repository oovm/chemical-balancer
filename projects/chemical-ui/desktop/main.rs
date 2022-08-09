#![allow(non_snake_case)]

use dioxus_desktop::{
    tao::{platform::windows::WindowBuilderExtWindows, window::Icon},
    Config, WindowBuilder,
};
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use chemical_ui::Editor;

fn main() {
    let icon = Icon::from_rgba(include_bytes!("default_icon.bin").to_vec(), 192, 192).unwrap();
    let win = WindowBuilder::new()
        .with_title("Chemical Balancer")
        .with_taskbar_icon(Some(icon.clone()))
        .with_window_icon(Some(icon.clone()))
        .with_content_protection(true);
    let cfg = Config::new().with_window(win);
    dioxus_desktop::launch_cfg(Editor, cfg);
}
