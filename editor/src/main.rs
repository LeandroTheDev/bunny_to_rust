//! Editor with your game connected to it as a plugin.
use fyrox::event_loop::EventLoop;
use fyroxed_base::{Editor, StartupData};
use rust_project::GameConstructor;

fn main() {
    let event_loop = EventLoop::new();
    let mut editor = Editor::new(
        &event_loop,
        Some(StartupData {
            working_directory: Default::default(),
            scene: "data/scenes/level1.rgs".into(),
        }),
    );
    editor.add_game_plugin(GameConstructor);
    editor.run(event_loop)
}
