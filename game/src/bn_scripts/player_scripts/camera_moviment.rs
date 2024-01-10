use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
        TypeUuidProvider,
    },
    event::{DeviceEvent, Event},
    impl_component_provider,
    scene::transform::Transform,
    script::{ScriptContext, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct CameraMoviment {
    pub pitch: f32,
    pub yaw: f32,
}
impl CameraMoviment {
    //Mouse Detect Function Pitch
    pub fn process_camera_moviment_pitch(&mut self, event: &Event<()>, camera_node: &mut Transform) {
        match event {
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    self.yaw -= delta.0 as f32;
                    self.pitch = (self.pitch + delta.1 as f32).clamp(-90.0, 90.0);
                    camera_node.set_rotation(UnitQuaternion::from_axis_angle(
                        &Vector3::x_axis(),
                        //The 3 is mouse sensitivy
                        self.pitch.to_radians() / 3.,
                    ));
                }
            }
            _ => (),
        }
    }
    //Mouse Detect Function Yaw
    pub fn process_camera_moviment_yaw(camera_node: &mut Transform) {
        
    }
    pub fn get_pitch(&mut self) -> f32 {
        return self.pitch;
    }
    pub fn get_yaw(&mut self) -> f32 {
        return self.yaw;
    }
}

impl_component_provider!(CameraMoviment);

impl TypeUuidProvider for CameraMoviment {
    fn type_uuid() -> Uuid {
        uuid!("9a9be198-92d4-4693-bd4a-0070d73b95ac")
    }
}

impl ScriptTrait for CameraMoviment {
    fn on_init(&mut self, _context: &mut ScriptContext) {
        // Declaring variables
        self.pitch = 0.0; //Vertical View
        self.yaw = 0.0; //Horizontal View
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Enable mouse detection
        let camera_node = context.scene.graph[context.handle].local_transform_mut();
        self.process_camera_moviment_pitch(event, camera_node);
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
