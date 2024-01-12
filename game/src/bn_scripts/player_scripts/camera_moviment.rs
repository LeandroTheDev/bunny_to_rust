use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        impl_component_provider,
        log::Log,
        pool::Handle,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
        TypeUuidProvider,
    },
    event::{DeviceEvent, Event},
    scene::node::Node,
    script::{ScriptContext, ScriptMessageContext, ScriptMessagePayload, ScriptTrait},
};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct CameraMoviment {
    pub pitch: f32,
    pub yaw: f32,
    player_node: Handle<Node>,
}
impl CameraMoviment {
    //Mouse Detect Function Pitch
    fn process_camera_moviment_pitch(&mut self, event: &Event<()>, camera_node: &mut Node) {
        match event {
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    self.pitch = (self.pitch + delta.1 as f32).clamp(-180.0, 180.0);
                    camera_node.local_transform_mut().set_rotation(
                        UnitQuaternion::from_axis_angle(
                            &Vector3::x_axis(),
                            self.pitch.to_radians() / 3.,
                        ),
                    );
                }
            }
            _ => (),
        }
    }
    //Mouse Detect Function Yaw
    fn process_camera_moviment_yaw(&mut self, event: &Event<()>, player_node: &mut Node) {
        match event {
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::MouseMotion { delta } = event {
                    self.yaw -= delta.0 as f32;
                    player_node.local_transform_mut().set_rotation(
                        UnitQuaternion::from_axis_angle(
                            &Vector3::y_axis(),
                            self.yaw.to_radians() / 3.,
                        ),
                    );
                }
            }
            _ => (),
        }
    }
    // Reset the view to default of level
    fn reset_camera_moviment_with_script_message(&mut self, context: &mut ScriptMessageContext) {
        self.yaw = 180. * 3.;
        self.pitch = 0.;
        {
            let player_node = &mut context.scene.graph[self.player_node];
            player_node
                .local_transform_mut()
                .set_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::y_axis(),
                    self.yaw.to_radians(),
                ));
        }
        {
            let camera_node = &mut context.scene.graph[context.handle];
            camera_node
                .local_transform_mut()
                .set_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::x_axis(),
                    self.pitch.to_radians(),
                ));
        }
    }

    fn reset_camera_moviment(&mut self, context: &mut ScriptContext) {
        self.yaw = 180. * 3.;
        self.pitch = 0.;
        {
            let player_node = &mut context.scene.graph[self.player_node];
            player_node
                .local_transform_mut()
                .set_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::y_axis(),
                    self.yaw.to_radians(),
                ));
        }
        {
            let camera_node = &mut context.scene.graph[context.handle];
            camera_node
                .local_transform_mut()
                .set_rotation(UnitQuaternion::from_axis_angle(
                    &Vector3::x_axis(),
                    self.pitch.to_radians(),
                ));
        }
    }
}

impl_component_provider!(CameraMoviment);

impl TypeUuidProvider for CameraMoviment {
    fn type_uuid() -> Uuid {
        uuid!("9a9be198-92d4-4693-bd4a-0070d73b95ac")
    }
}

impl ScriptTrait for CameraMoviment {
    fn on_start(&mut self, context: &mut ScriptContext) {
        // Subscribing messages
        context
            .message_dispatcher
            .subscribe_to::<&str>(context.handle);
        self.reset_camera_moviment(context);
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Enable mouse detection
        // Process Vertical View
        self.process_camera_moviment_pitch(event, &mut context.scene.graph[context.handle]);
        // Process Horizontal View
        self.process_camera_moviment_yaw(event, &mut context.scene.graph[self.player_node]);
    }

    fn on_message(
        &mut self,
        message: &mut dyn ScriptMessagePayload,
        context: &mut ScriptMessageContext,
    ) {
        // React to message.
        if let Some(data) = message.downcast_ref::<&str>() {
            if data == &"reset_camera" {
                self.reset_camera_moviment_with_script_message(context);
            }
        }
    }
}
