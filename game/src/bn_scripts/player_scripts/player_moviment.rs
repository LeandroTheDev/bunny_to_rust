use fyrox::{
    core::{
        algebra::{ArrayStorage, Const, Matrix, UnitQuaternion, Vector3},
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
        TypeUuidProvider,
    },
    event::{ElementState, Event, WindowEvent},
    impl_component_provider,
    keyboard::{KeyCode, PhysicalKey},
    script::{ScriptContext, ScriptTrait},
};

use super::camera_moviment::CameraMoviment;

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct PlayerMoviment {
    // Player Directions
    position_x: bool,
    position_x_negative: bool,
    position_z: bool,
    position_z_negative: bool,
    jump: bool,
    // Flags
    ticks_dessaceleration: i32,
    ticks_jump_cooldown: i32,
    ticks_reset_cooldown: i32,
    acceleration: f32,
    straffing: bool,
    old_mouse_position: f32,
}
impl PlayerMoviment {
    //Keyboard Detect Function
    pub fn process_input_event(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent { event, .. } => {
                if let WindowEvent::KeyboardInput { event, .. } = event {
                    let pressed = event.state == ElementState::Pressed;
                    if let PhysicalKey::Code(code) = event.physical_key {
                        match code {
                            KeyCode::KeyW => self.position_z = pressed,
                            KeyCode::KeyS => self.position_z_negative = pressed,
                            KeyCode::KeyA => self.position_x = pressed,
                            KeyCode::KeyD => self.position_x_negative = pressed,
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
        &mut self,
        velocity: Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>,
        is_not_air: bool,
        is_frontal_collide: bool,
        is_pressing_s: bool,
        acceleration_mouse: f32,
    ) -> Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>> {
        let mut base_velocity = velocity;
        for i in 0..3 {
            //If is in the air and moving the camera
            if i != 1
                && !is_not_air
                && acceleration_mouse != 0.
                && !is_pressing_s
                && !is_frontal_collide
            {
                let acceleration: f32;
                //Determines the maximum speed earned
                if self.acceleration <= 5. {
                    if acceleration_mouse >= 0.02 {
                        acceleration = 0.02;
                    } else {
                        acceleration = acceleration_mouse;
                    }
                } else if self.acceleration <= 5. {
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
                self.ticks_dessaceleration = 0;
                self.acceleration += acceleration;
                base_velocity[i] *= self.acceleration;
            //If is only in the air
            } else if i != 1 && !is_not_air && !is_pressing_s && !is_frontal_collide {
                self.ticks_dessaceleration = 0;
                base_velocity[i] *= self.acceleration;

            //Lowering the acceleartion conditions
            } else if i != 1 {
                //If is on the ground
                if is_not_air && !is_frontal_collide {
                    self.ticks_dessaceleration += 1;
                    if self.ticks_dessaceleration >= 20 {
                        self.ticks_dessaceleration = 20;
                        self.acceleration /= 1.03;
                        if self.acceleration <= 1. {
                            self.acceleration = 1.;
                            self.straffing = false;
                        }
                    }
                    base_velocity[i] *= self.acceleration;
                    //If is fronta colission
                } else if is_frontal_collide {
                    self.acceleration /= 3.;
                    if self.acceleration <= 1. {
                        self.acceleration = 1.;
                        self.straffing = false;
                    }
                    base_velocity[i] *= self.acceleration;
                    //If pressing S
                } else if is_pressing_s {
                    self.acceleration /= 2.;
                    if self.acceleration <= 1. {
                        self.acceleration = 1.;
                        self.straffing = false;
                    }
                    base_velocity[i] *= self.acceleration;
                } else {
                    base_velocity[i] *= self.acceleration;
                }
            }
        }
        return base_velocity;
    }

    //Reset Scene
    pub fn reset_player(&mut self, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => {
                if let WindowEvent::KeyboardInput { event, .. } = event {
                    let pressed = event.state == ElementState::Pressed;
                    if let PhysicalKey::Code(code) = event.physical_key {
                        match code {
                            KeyCode::KeyR => {
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

impl_component_provider!(PlayerMoviment);

impl TypeUuidProvider for PlayerMoviment {
    fn type_uuid() -> Uuid {
        uuid!("5e5f5d29-a9a9-447e-8010-9f413d9f6efb")
    }
}

impl ScriptTrait for PlayerMoviment {
    fn on_init(&mut self, context: &mut ScriptContext) {
        let node = context.scene.graph.find_by_name_from_root("name").unwrap().0;
        // let test = context.scene.graph.try_get_script_of_mut::<CameraMoviment>().unwrap();
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        //Keyboard Observer
        self.process_input_event(event);
        let reset_player = self.reset_player(event);
        //Reset Player Observer
        if reset_player && self.ticks_reset_cooldown > 30 {
            // Borrow rigid body node.
            let body = context.scene.graph[context.handle].as_rigid_body_mut();
            self.ticks_reset_cooldown = 0;
            self.acceleration = 1.;
            body.set_lin_vel(Vector3::new(0.0, 0.0, 0.0));
            context.scene.graph[context.handle]
                .local_transform_mut()
                .set_position(Vector3::new(0.082, 3.15, 8.897));
            //The 3 is mouse sensitivy
            CameraMoviment::YAW * 3;
            CameraMoviment::PITCH = 0.;
        }
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        //Movement Player Update
        // Borrow rigid body node.
        let body = context.scene.graph[context.handle].as_rigid_body_mut();
        // Keep only vertical velocity, and drop horizontal.
        let mut velocity = Vector3::new(0.0, body.lin_vel().y, 0.0);
        let mut dessacelerate: bool = false;
        let mut mouse_accelerate: f32 = 0.;

        // Change the velocity depending on the keys pressed.
        if self.position_z || self.straffing {
            // If we moving forward then add "look" vector of the body.
            self.straffing = true;
            velocity += body.look_vector() * 2.;
        }
        if self.position_z_negative {
            // If we moving backward then subtract "look" vector of the body.
            velocity -= body.look_vector() * 2.;
            dessacelerate = true;
        }
        if self.position_x {
            // If we moving left then add "side" vector of the body.
            velocity += body.side_vector() * 2.;
        }
        if self.position_x_negative {
            // If we moving right then subtract "side" vector of the body.
            velocity -= body.side_vector() * 2.;
        }
        if self.jump && foot_collider::is_on_air && self.ticks_jump_cooldown <= 3 {
            //Check if is the first tick
            if self.ticks_jump_cooldown == -1 {
                self.ticks_jump_cooldown = 0;
            }
            // If we moving up add "up" vector of the body
            velocity += body.up_vector() * 2.;
        }
        //Cooldown the Jump Ticks
        if self.ticks_jump_cooldown >= 0 && self.ticks_jump_cooldown <= 20 {
            self.ticks_jump_cooldown += 1;
        } else if self.ticks_jump_cooldown > 20 {
            self.ticks_jump_cooldown = -1;
        }
        if self.old_mouse_position != CameraMoviment::YAW.to_radians() {
            //Calculates the mouse velocity
            let mut _player_mouse_position: f32 = 0.;
            //Negative to Positive
            if CameraMoviment::YAW.to_radians() < 0. {
                _player_mouse_position = CameraMoviment::YAW.to_radians().abs();
            } else {
                _player_mouse_position = CameraMoviment::YAW.to_radians();
            }
            //Difference between
            if _player_mouse_position != self.old_mouse_position {
                if _player_mouse_position < self.old_mouse_position {
                    mouse_accelerate = self.old_mouse_position - _player_mouse_position;
                } else {
                    mouse_accelerate = _player_mouse_position - self.old_mouse_position;
                }
            }
            self.old_mouse_position = _player_mouse_position
        }
        // Finally new linear velocity.
        body.set_lin_vel(self.velocity(
            velocity,
            FootCollider::IS_ON_AIR,
            FrontalCollider::IS_FRONTAL_COLLIDE,
            dessacelerate,
            mouse_accelerate,
        ));
        //Horizontal Mouse View Update
        context.scene.graph[context.handle]
            .local_transform_mut()
            .set_rotation(UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                CameraMoviment::YAW.to_radians() / 3,
            ));
        //Reset Tick Cooldown
        if self.ticks_reset_cooldown <= 30 {
            self.ticks_reset_cooldown += 1;
        }
    }

    fn id(&self) -> Uuid {
        Self::type_uuid()
    }
}
