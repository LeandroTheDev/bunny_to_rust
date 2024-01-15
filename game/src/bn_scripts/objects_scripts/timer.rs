use fyrox::{
    core::{
        impl_component_provider,
        log::Log,
        reflect::prelude::*,
        uuid::{uuid, Uuid},
        visitor::prelude::*,
        TypeUuidProvider,
    },
    gui::{message::MessageDirection, text::TextMessage},
    script::{ScriptContext, ScriptTrait},
};

use crate::{bn_scripts::GAME_PAUSED, Game};

#[derive(Visit, Reflect, Default, Debug, Clone)]
pub struct Timer {
    milliseconds: f32,
    seconds: i16,
    timer: String,
    pub finished: bool,
    pub stop: bool,
}

impl Timer {
    /// Resets the timer to 0 and make the timer stopped
    pub fn reset_timer(&mut self) {
        self.milliseconds = 0.0;
        self.seconds = 0;
        self.stop = true;
        self.finished = false;
        self.timer = "0:0".to_string();
    }
}

impl_component_provider!(Timer);

impl TypeUuidProvider for Timer {
    fn type_uuid() -> Uuid {
        uuid!("a68ae647-c5d9-46f1-912f-bb7f8b07f1a8")
    }
}

impl ScriptTrait for Timer {
    fn on_init(&mut self, _context: &mut ScriptContext) {
        self.milliseconds = 0.0;
        self.seconds = 0;
        self.stop = true;
        self.timer = "0:0".to_string();
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        // Verify if timer is stopped
        if !self.stop && !self.finished {
            // Incrementing the timer
            self.milliseconds += context.dt * 1000.0;
            // Check if milliseconds passed a second
            if self.milliseconds > 1000.0 {
                // Converts in string
                let mut string_milliseconds = self.milliseconds.to_string();

                // Takes the first character in string
                if let Some(first_character) = string_milliseconds.chars().next() {
                    // Converts the first character into i16 and plus the seconds
                    // this is necessary for hardware freezings more than 1 second
                    self.seconds += first_character.to_digit(10).unwrap_or(0) as i16;
                    // Removes the first character
                    string_milliseconds.remove(0);
                    // Converts to f32 again
                    self.milliseconds = string_milliseconds.parse().unwrap_or(self.milliseconds);
                }
                // Error message
                else {
                    Log::err("Timer Error: cannot find the first character of milliseconds");
                }
            }
            // Converts the timer into readable string
            self.timer = format!("{}:{}", self.seconds, self.milliseconds as i16).to_string();
            // Update the Text Widget
            context.user_interface.send_message(TextMessage::text(
                context.plugins.of_type_ref::<Game>().unwrap().timer_text,
                MessageDirection::ToWidget,
                self.timer.to_string(),
            ));
        }
    }
}
