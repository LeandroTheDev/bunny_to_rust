//! Scripts
pub mod player_scripts;
use player_scripts::{camera_movement::CameraMovement, player_collider::PlayerCollider};
use player_scripts::player_movement::PlayerMovement;
//Game project.
use fyrox::{
    core::pool::Handle,
    event::Event,
    event_loop::ControlFlow,
    gui::message::UiMessage,
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::{loader::AsyncSceneLoader, Scene},
    utils::log::Log,
};

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    //Scripts Register
    fn register(&self, _context: PluginRegistrationContext) {
        _context
            .serialization_context
            .script_constructors
            .add::<PlayerMovement>("Player Movement");
        _context
            .serialization_context
            .script_constructors
            .add::<CameraMovement>("Camera Movement");
        _context
            .serialization_context
            .script_constructors
            .add::<PlayerCollider>("Player Collider");
    }

    //Game declaration
    fn create_instance(
        &self,
        override_scene: Handle<Scene>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        Box::new(Game::new(override_scene, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
    loader: Option<AsyncSceneLoader>,
}

impl Game {
    //Loading Scene
    pub fn new(override_scene: Handle<Scene>, context: PluginContext) -> Self {
        let mut loader = None;
        let scene = if override_scene.is_some() {
            override_scene
        } else {
            loader = Some(AsyncSceneLoader::begin_loading(
                "data/scenes/level1.rgs".into(),
                context.serialization_context.clone(),
                context.resource_manager.clone(),
            ));
            Default::default()
        };

        Self { scene, loader }
    }
}

impl Plugin for Game {
    //After load
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    //Loop
    fn update(&mut self, context: &mut PluginContext, _control_flow: &mut ControlFlow) {
        if let Some(loader) = self.loader.as_ref() {
            if let Some(result) = loader.fetch_result() {
                match result {
                    Ok(scene) => {
                        self.scene = context.scenes.add(scene);
                    }
                    Err(err) => Log::err(err),
                }
            }
        }
        // Add your global update code here.
    }

    //Dont know
    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
        _control_flow: &mut ControlFlow,
    ) {
        // Do something on OS event here.
    }

    //Dont know
    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
        _control_flow: &mut ControlFlow,
    ) {
        // Handle UI events here.
    }
}
