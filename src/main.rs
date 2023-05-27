mod world;
//Graphics Card
extern crate graphics;
extern crate opengl_graphics;
use opengl_graphics::{GlGraphics, OpenGL};

//Piston Dependecie
extern crate piston;
use piston::event_loop::{EventSettings, Events};

//Creating Window Dependecie with Piston
use piston::{RenderEvent, Size, WindowSettings};
extern crate glutin_window;
use glutin_window::GlutinWindow;


//Initialize
fn main() {
    //Call the Graphics Card
    let opengl = OpenGL::V3_2;

    //Call The Window
    let settings = WindowSettings::new(
        "Rust Project",
        Size {
            width: 1280.,
            height: 860.,
        },
    )
    .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    //Events
    let mut events = Events::new(EventSettings::new());

    //Initialize Graphics Card
    let mut gl = GlGraphics::new(opengl);

    let map = world::consctruct_map();

    //Event Loop Every Frame
    while let Some(e) = events.next(&mut window) {
        //Render Graphics
        if let Some(render_args) = e.render_args() {
            gl.draw(render_args.viewport(), |c, g| {
                for i in 0..world::WORLD_SIZE {
                    for j in 0..world::WORLD_SIZE {
                        let pos: [f64; 4] = [
                            world::PIXEL_SIZE * i as f64,
                            world::PIXEL_SIZE * j as f64,
                            world::PIXEL_SIZE * (i + 1) as f64,
                            world::PIXEL_SIZE * (j + 1) as f64,
                        ];
                        graphics::Rectangle::new(map[i as usize][j as usize].colour).draw(
                            pos,
                            &c.draw_state,
                            c.transform,
                            g,
                        );
                    }
                }
            });
        }

        //Simple Function
        println!("Rendering");
    }
}
