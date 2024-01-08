//Bunny to Rust Dependencies
pub mod bn_engine;
pub mod bn_scripts;

use bn_scripts::player_scripts::{
    camera_moviment::CameraMoviment, foot_collider::FootCollider,
    frontal_collider::FrontalCollider, player_hand::PlayerHand, player_moviment::PlayerMoviment,
};
//Dependencies
use fyrox::{
    core::pool::Handle,
    event::Event,
    gui::message::UiMessage,
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use std::path::Path;

//GameControler Instance
pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, context: PluginRegistrationContext) {
        context
            .serialization_context
            .script_constructors
            .add::<PlayerMoviment>("Player Moviment");
        context
            .serialization_context
            .script_constructors
            .add::<CameraMoviment>("Camera Moviment");
        context
            .serialization_context
            .script_constructors
            .add::<PlayerHand>("Player Hand");
        context
            .serialization_context
            .script_constructors
            .add::<FootCollider>("Foot Collider");
        context
            .serialization_context
            .script_constructors
            .add::<FrontalCollider>("Frontal Collider");
    }

    fn create_instance(&self, scene_path: Option<&str>, context: PluginContext) -> Box<dyn Plugin> {
        Box::new(Game::new(scene_path, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
}

impl Game {
    pub fn new(scene_path: Option<&str>, context: PluginContext) -> Self {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scenes/scenario.rgs"));
        Self {
            scene: Handle::NONE,
        }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {
        // Do something on OS event here.
    }

    fn on_ui_message(&mut self, _context: &mut PluginContext, _message: &UiMessage) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, _path: &Path, context: &mut PluginContext) {
        if self.scene.is_some() {
            context.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        _path: &Path,
        scene: Handle<Scene>,
        _data: &[u8],
        _context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}
