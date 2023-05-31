use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        reflect::{FieldInfo, Reflect},
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    event::{DeviceEvent, Event},
    impl_component_provider,
    scene::node::TypeUuidProvider,
    script::{ScriptContext, ScriptTrait},
};

use super::mouse_sensitivy;
//Camera Movement Script
#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct CameraMovement {
    pub pitch: f32,
    pub yaw: f32,
}
impl CameraMovement {
    //Mouse Detect Function
    pub fn process_input_event(&mut self, event: &Event<()>) {
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
pub static mut PLAYER_CAMERA: CameraMovement = CameraMovement {
    pitch: 0.0,
    yaw: 180.0 * 3.,
};
//Declaration
impl_component_provider!(CameraMovement);

//ID
impl TypeUuidProvider for CameraMovement {
    fn type_uuid() -> Uuid {
        uuid!("9a9be198-92d4-4693-bd4a-0070d73b95ac")
    }
}

//Loops
impl ScriptTrait for CameraMovement {
    //Event Checker
    fn on_os_event(&mut self, event: &Event<()>, _context: &mut ScriptContext) {
        unsafe { PLAYER_CAMERA.process_input_event(event) };
    }
    //Frame Update
    fn on_update(&mut self, context: &mut ScriptContext) {
        //Mouse Vertical View
        context.scene.graph[context.handle]
            .local_transform_mut()
            .set_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::x_axis(),
                unsafe { PLAYER_CAMERA.pitch.to_radians() / mouse_sensitivy },
            ));
            // context.scene.graph[context.handle]
            // .local_transform_mut()
            // .set_rotation(UnitQuaternion::from_matrix(&Matrix3::new(
            //     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            // )));
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
