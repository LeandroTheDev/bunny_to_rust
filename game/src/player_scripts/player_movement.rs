use fyrox::{
    core::{
        algebra::{ArrayStorage, ComplexField, Const, Matrix, UnitQuaternion, Vector3},
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
use crate::player_scripts::camera_movement;
//Use player_collider script to know if the player are in the air
use crate::player_scripts::player_collider;

//Player Movement Script
#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct PlayerMovement {
    position_x: bool,
    position_x_negative: bool,
    position_z: bool,
    position_z_negative: bool,
    jump: bool,
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
                            VirtualKeyCode::Space => {
                                self.jump = input.state == ElementState::Pressed;
                            }
                            VirtualKeyCode::R => {
                                self.jump = input.state == ElementState::Pressed;
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        }
    }

    //Velocity calculator
    pub fn velocity(
        velocity: Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>,
        is_on_air: bool,
        is_pressing_s: bool,
        acceleration_mouse: f32,
    ) -> Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>> {
        let mut base_velocity = velocity;
        for i in 0..3 {
            //If is in the air pressing A D
            if i != 1 && !is_on_air && acceleration_mouse != 0. && !is_pressing_s {
                unsafe {
                    let acceleration: f32;
                    //Determines the maximum speed earned
                    if PLAYER_ACCELERATION <= 5. {
                        if acceleration_mouse >= 0.02 {
                            acceleration = 0.02;
                        } else {
                            acceleration = acceleration_mouse;
                        }
                    } else if PLAYER_ACCELERATION <= 5. {
                        if acceleration_mouse >= 0.02 {
                            acceleration = 0.02;
                        } else {
                            acceleration = acceleration_mouse;
                        }
                    } else {
                        if acceleration_mouse >= 0.005 {
                            acceleration = 0.005;
                        } else {
                            acceleration = acceleration_mouse;
                        }
                    }
                    PLAYER_TICKS = 0;
                    PLAYER_ACCELERATION += acceleration;
                    base_velocity[i] *= PLAYER_ACCELERATION;
                }
            //If is in the air without pressing A D
            } else if i != 1 && !is_on_air && !is_pressing_s {
                unsafe {
                    PLAYER_TICKS = 0;
                    base_velocity[i] *= PLAYER_ACCELERATION;
                }
            //If is on the ground without pressing A D
            } else if i != 1 {
                unsafe {
                    PLAYER_TICKS += 1;
                    if PLAYER_TICKS >= 5 || is_pressing_s {
                        PLAYER_TICKS = 5;
                        if is_pressing_s {
                            PLAYER_ACCELERATION /= 2.;
                        } else {
                            PLAYER_ACCELERATION /= 1.03;
                        }
                        if PLAYER_ACCELERATION <= 1. {
                            PLAYER_ACCELERATION = 1.;
                            PLAYER_STRAFFING = false;
                        }
                    }
                    base_velocity[i] *= PLAYER_ACCELERATION;
                }
            }
        }
        return base_velocity;
    }

    //Reset Scene
    pub fn reset_player(event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => {
                if let WindowEvent::KeyboardInput { input, .. } = event {
                    if let Some(key_code) = input.virtual_keycode {
                        match key_code {
                            VirtualKeyCode::R => {
                                return true;
                            }
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        }
        return false;
    }
}
static mut PLAYER_MOVEMENT: PlayerMovement = PlayerMovement {
    position_x: false,
    position_x_negative: false,
    position_z: false,
    position_z_negative: false,
    jump: false,
};
static mut PLAYER_TICKS: i32 = 0;
static mut PLAYER_ACCELERATION: f32 = 1.0;
static mut PLAYER_STRAFFING: bool = false;
static mut PLAYER_OLD_MOUSE_POSITION: f32 = 0.0;
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
    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        //Keyboard Observer
        unsafe { PLAYER_MOVEMENT.process_input_event(event) };
        let reset_player = PlayerMovement::reset_player(event);
        if reset_player {
            unsafe { PLAYER_ACCELERATION = 1. };
            context.scene.graph[context.handle]
                .local_transform_mut()
                .set_position(Vector3::new(0.088, 3.239, 0.875));
        }
    }

    //Frame Update
    fn on_update(&mut self, context: &mut ScriptContext) {
        //Movement Player Update
        if true {
            // Borrow rigid body node.
            let body = context.scene.graph[context.handle].as_rigid_body_mut();
            // Keep only vertical velocity, and drop horizontal.
            let mut velocity = Vector3::new(0.0, body.lin_vel().y, 0.0);
            let mut accelerate = false;
            let mut dessacelerate: bool = false;
            let mut mouse_accelerate: f32 = 0.;

            // Change the velocity depending on the keys pressed.
            if unsafe { PLAYER_MOVEMENT.position_z || PLAYER_STRAFFING } {
                // If we moving forward then add "look" vector of the body.
                unsafe { PLAYER_STRAFFING = true };
                velocity += body.look_vector() * 2.;
            }
            if unsafe { PLAYER_MOVEMENT.position_z_negative } {
                // If we moving backward then subtract "look" vector of the body.
                velocity -= body.look_vector() * 2.;
                dessacelerate = true;
            }
            if unsafe { PLAYER_MOVEMENT.position_x } {
                // If we moving left then add "side" vector of the body.
                velocity += body.side_vector() * 2.;
                accelerate = true;
            }
            if unsafe { PLAYER_MOVEMENT.position_x_negative } {
                // If we moving right then subtract "side" vector of the body.
                velocity -= body.side_vector() * 2.;
                accelerate = true;
            }
            if unsafe { PLAYER_MOVEMENT.jump && player_collider::IS_ON_AIR } {
                // If we moving up add "up" vector of the body
                velocity += body.up_vector() * 1.5;
            }
            if unsafe {
                PLAYER_OLD_MOUSE_POSITION != camera_movement::PLAYER_CAMERA.yaw.to_radians()
            } {
                unsafe {
                    //Calculates the mouse velocity
                    let mut _player_mouse_position: f32 = 0.;
                    //Negative to Positive
                    if camera_movement::PLAYER_CAMERA.yaw.to_radians() < 0. {
                        _player_mouse_position =
                            camera_movement::PLAYER_CAMERA.yaw.to_radians().abs();
                    } else {
                        _player_mouse_position = camera_movement::PLAYER_CAMERA.yaw.to_radians();
                    }
                    //Difference between
                    if _player_mouse_position != PLAYER_OLD_MOUSE_POSITION {
                        if _player_mouse_position < PLAYER_OLD_MOUSE_POSITION {
                            mouse_accelerate = PLAYER_OLD_MOUSE_POSITION - _player_mouse_position;
                        } else {
                            mouse_accelerate = _player_mouse_position - PLAYER_OLD_MOUSE_POSITION;
                        }
                    }
                    PLAYER_OLD_MOUSE_POSITION = _player_mouse_position
                }
            }
            // Finally new linear velocity.
            body.set_lin_vel(PlayerMovement::velocity(
                velocity,
                unsafe { player_collider::IS_ON_AIR },
                dessacelerate,
                mouse_accelerate,
            ));
        }
        //Horizontal Mouse View Update
        context.scene.graph[context.handle]
            .local_transform_mut()
            .set_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                unsafe { camera_movement::PLAYER_CAMERA.yaw.to_radians() / 2. },
            ));
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
