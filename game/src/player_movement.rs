use fyrox::{
    core::{
        algebra::{Matrix3, UnitQuaternion, Vector3},
        reflect::{FieldInfo, Reflect},
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    engine::resource_manager::ResourceManager,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    impl_component_provider,
    scene::node::TypeUuidProvider,
    script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use crate::camera_movement;

//Player Movement Script
#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct PlayerMovement {
    position_x: bool,
    position_x_negative: bool,
    position_z: bool,
    position_z_negative: bool,
}
impl PlayerMovement {
    //Keyboard Detect Function
    pub fn process_input_event(&mut self, event: &Event<()>, context: &ScriptContext) {
        match event {
            Event::WindowEvent { event, .. } => {
                if let WindowEvent::KeyboardInput { input, .. } = event {
                    if let Some(key_code) = input.virtual_keycode {
                        match key_code {
                            VirtualKeyCode::A => {
                                self.position_x = input.state == ElementState::Pressed;
                            }
                            VirtualKeyCode::D => {
                                self.position_x_negative = input.state == ElementState::Pressed;
                            }
                            VirtualKeyCode::W => {
                                self.position_z = input.state == ElementState::Pressed;
                            }
                            VirtualKeyCode::S => {
                                self.position_z_negative = input.state == ElementState::Pressed;
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
static mut PLAYER_MOVEMENT: PlayerMovement = PlayerMovement {
    position_x: false,
    position_x_negative: false,
    position_z: false,
    position_z_negative: false,
};
//Declaration
impl_component_provider!(PlayerMovement);

//ID
impl TypeUuidProvider for PlayerMovement {
    fn type_uuid() -> Uuid {
        uuid!("5e5f5d29-a9a9-447e-8010-9f413d9f6efb")
    }
}

//Loops
impl ScriptTrait for PlayerMovement {
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
        //Keyboard Observer
        unsafe { PLAYER_MOVEMENT.process_input_event(event, context) };
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        //Movement Player
        unsafe {
            let position_x = context.scene.graph[context.handle].global_position()[0];
            let position_y = context.scene.graph[context.handle].global_position()[1];
            let position_z = context.scene.graph[context.handle].global_position()[2];
            if PLAYER_MOVEMENT.position_x
                || PLAYER_MOVEMENT.position_x_negative
                || PLAYER_MOVEMENT.position_z
                || PLAYER_MOVEMENT.position_z_negative
            {
                //left
                if PLAYER_MOVEMENT.position_x {
                    context.scene.graph[context.handle]
                        .local_transform_mut()
                        .set_position(Vector3::new(position_x + 0.03, position_y, position_z));
                }
                //right
                if PLAYER_MOVEMENT.position_x_negative {
                    context.scene.graph[context.handle]
                        .local_transform_mut()
                        .set_position(Vector3::new(position_x - 0.03, position_y, position_z));
                }
                //forward
                if PLAYER_MOVEMENT.position_z {
                    context.scene.graph[context.handle]
                        .local_transform_mut()
                        .set_position(Vector3::new(position_x, position_y, position_z + 0.03));
                }
                //backward
                if PLAYER_MOVEMENT.position_z_negative {
                    context.scene.graph[context.handle]
                        .local_transform_mut()
                        .set_position(Vector3::new(position_x, position_y, position_z - 0.03));
                }
            }
        }

        //Horizontal Mouse View
        context.scene.graph[context.handle]
            .local_transform_mut()
            .set_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                unsafe { camera_movement::player_camera.yaw.to_radians() },
            ));
        //Rotation fix
        // context.scene.graph[context.handle]
        //     .local_transform_mut()
        //     .set_rotation(UnitQuaternion::from_matrix(&Matrix3::new(
        //         0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        //     )));
    }

    fn restore_resources(&mut self, resource_manager: ResourceManager) {
        // Restore resource handles here.
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
