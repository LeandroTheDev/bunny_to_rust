use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        reflect::{FieldInfo, Reflect},
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    engine::resource_manager::ResourceManager,
    event::{DeviceEvent, Event},
    impl_component_provider,
    scene::node::TypeUuidProvider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct CameraMovement {
    pub pitch: f32,
    pub yaw: f32,
}
impl CameraMovement {
    //Mouse Detect Function
    pub fn process_input_event(&mut self, event: &Event<()>, context: &ScriptContext) {
        match event {
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    self.yaw -= delta.0 as f32;
                    self.pitch = (self.pitch + delta.1 as f32).clamp(-90.0, 90.0);
                }
            }
            _ => (),
        }
    }
}
pub static mut player_camera: CameraMovement = CameraMovement {
    pitch: 0.0,
    yaw: 0.0,
};

impl_component_provider!(CameraMovement);

impl TypeUuidProvider for CameraMovement {
    fn type_uuid() -> Uuid {
        uuid!("9a9be198-92d4-4693-bd4a-0070d73b95ac")
    }
}

impl ScriptTrait for CameraMovement {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        unsafe { player_camera.process_input_event(event, context) };
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        //Mouse Vertical View
        context.scene.graph[context.handle]
            .local_transform_mut()
            .set_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::x_axis(),
                unsafe { player_camera.pitch.to_radians() },
            ));
    }

    fn restore_resources(&mut self, resource_manager: ResourceManager) {
        // Restore resource handles here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
