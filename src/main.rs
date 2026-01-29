#![allow(dead_code, unreachable_code, unused_imports)]

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod stellar_core;
mod stellar_utils;
mod procedural_generation;
mod ui;
mod game;

fn main() {

    App::new()
        .add_plugins((
                DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Stellar"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        //resolution: Vec2::new(512., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
                ShapePlugin,
                bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
                game::GamePlugin,

            ))
        .add_systems(Startup, set_window_icon)
        .run();
}

fn set_window_icon(windows: NonSend<bevy::winit::WinitWindows>) {
    //here we use the `image` crate to load our icon data from a png file
    //this is not a very bevy-native solution, but it will do
    //Winit doesn't work on linux. there we can use .desktop files to set icons
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    //do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}