mod player;
mod world;
//Graphics Card
extern crate graphics;
extern crate opengl_graphics;
use graphics::{CharacterCache, Transformed};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};

//Piston Dependecie
extern crate piston;
use piston::event_loop::{EventSettings, Events};

//Creating Window Dependecie with Piston
use piston::{RenderEvent, Size, WindowSettings, ButtonState, Button, Key, ButtonEvent};
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

    //Initialize Map Construction
    let map = world::consctruct_map();

    //Player Initialize
    let mut player: player::Object = player::Object::new(0, 0, '@', world::RED);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    //Event Loop Every Frame
    while let Some(event) = events.next(&mut window) {
        //Render Graphics
        if let Some(render_args) = event.render_args() {
            //C = Context, G = opengl graphics
            gl.draw(render_args.viewport(), |c, g| {
                //World Renderization
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
                //Player Renderization
                let character = glyphs
                    .character(world::PIXEL_SIZE as u32, player.character)
                    .unwrap();
                graphics::Image::new_color(player.colour).draw(
                    character.texture,
                    &c.draw_state,
                    c.transform.trans(player.x as f64, player.y as f64),
                    g,
                );
            });
            //Keyboard Controls
            if let Some(k) = event.button_args() {
                if k.state == ButtonState::Press {
                    match k.button {
                        Button::Keyboard(Key::W) => player.y -= 32,
                        Button::Keyboard(Key::S) => player.y += 32,
                        Button::Keyboard(Key::A) => player.x -= 32,
                        Button::Keyboard(Key::D) => player.x += 32,
                        _ => (),
                    }
                }
            }
        }
    }
}
