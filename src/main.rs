//Piston Dependecie
extern crate piston;
use piston::event_loop::{EventLoop, EventSettings, Events};
use std::collections::HashMap;

//Creating Window Dependecie
use piston::WindowSettings;
extern crate glutin_window;
use glutin_window::GlutinWindow;

fn main() {
    //Call The Window
    let settings = WindowSettings::new("Rust Project", [1280, 820]).exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    //Colors
    let mut colors = HashMap::new();
    colors.insert("RED", [1.0, 0.0, 0.0, 1.0]);
    colors.insert("GREEN", [0.0, 1.0, 0.0, 1.0]);
    colors.insert("BLUE", [0.0, 0.0, 1.0, 1.0]);
    colors.insert("WHITE", [1.0; 4]);

    //Events
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        println!("Rendering");
    }
}
