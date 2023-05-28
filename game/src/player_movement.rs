use fyrox::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        reflect::{FieldInfo, Reflect},
        uuid::{uuid, Uuid},
        visitor::prelude::*,
    },
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    impl_component_provider,
    scene::node::TypeUuidProvider,
    script::{ScriptContext, ScriptTrait},
};
//Use camera_movement script to change horizontal rotation
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
    pub fn process_input_event(&mut self, event: &Event<()>) {
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
    //Event Checker
    fn on_os_event(&mut self, event: &Event<()>, _context: &mut ScriptContext) {
        //Keyboard Observer
        unsafe { PLAYER_MOVEMENT.process_input_event(event) };
    }

    //Frame Update
    fn on_update(&mut self, context: &mut ScriptContext) {
        //Movement Player Update
        if true {
            // Borrow rigid body node.
            let body = context.scene.graph[context.handle].as_rigid_body_mut();
            // Keep only vertical velocity, and drop horizontal.
            let mut velocity = Vector3::new(0.0, body.lin_vel().y, 0.0);

            // Change the velocity depending on the keys pressed.
            if unsafe { PLAYER_MOVEMENT.position_z } {
                // If we moving forward then add "look" vector of the body.
                velocity += body.look_vector();
            }
            if unsafe { PLAYER_MOVEMENT.position_z_negative } {
                // If we moving backward then subtract "look" vector of the body.
                velocity -= body.look_vector();
            }
            if unsafe { PLAYER_MOVEMENT.position_x } {
                // If we moving left then add "side" vector of the body.
                velocity += body.side_vector();
            }
            if unsafe { PLAYER_MOVEMENT.position_x_negative } {
                // If we moving right then subtract "side" vector of the body.
                velocity -= body.side_vector();
            }

            // Finally new linear velocity.
            body.set_lin_vel(velocity);
        }
        //Horizontal Mouse View Update
        context.scene.graph[context.handle]
            .local_transform_mut()
            .set_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                unsafe { camera_movement::PLAYER_CAMERA.yaw.to_radians() },
            ));
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
