#![windows_subsystem = "windows"]
//! Executor with your game connected to it as a plugin.
use fyrox::engine::executor::Executor;
use rust_project::GameConstructor;

fn main() {
    let mut executor = Executor::new();
    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}