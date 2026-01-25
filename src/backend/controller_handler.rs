use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use enigo::{Coordinate, Direction, Enigo, Keyboard, Mouse, Settings};
use gilrs::{Axis, Event, Gilrs};
use gilrs::EventType::{AxisChanged, ButtonPressed, ButtonReleased};
use crate::backend::config_manager::ProfileConfig;

const DEADZONE: f32 = 0.05;
const MOUSE_SPEED_MODIFIER: f32 = 0.5;

pub async fn handle_controller_input(gilrs: Arc<Mutex<Gilrs>>, profile_config: Arc<Mutex<ProfileConfig>>, is_handler_running: Arc<AtomicBool>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let active_gamepad_config_map = profile_config.lock().unwrap().get_gamepad_config_map(gilrs.clone());

    let (mouse_x_pix, mouse_y_pix) = enigo.location().unwrap_or((0, 0));
    let mut mouse_x_pos = mouse_x_pix as f32;
    let mut mouse_y_pos = mouse_y_pix as f32;
    let mut mouse_x_amt = 0.0;
    let mut mouse_y_amt = 0.0;

    while is_handler_running.load(Ordering::Relaxed) == true {
        // Examine new events
        while let Some(Event { id, event, time, .. }) = gilrs.lock().unwrap().next_event() {
            // TODO: Handle when Gilrs find a new GamepadId that's missing from the map!
            let agc = active_gamepad_config_map.get(&id).unwrap();

            if let ButtonPressed(btn, _) = event {
                if let Some(key) = agc.get_key(&btn) {
                    enigo.key(*key, Direction::Press).unwrap();
                }
            } else if let ButtonReleased(btn, _) = event {
                if let Some(key) = agc.get_key(&btn) {
                    enigo.key(*key, Direction::Release).unwrap();
                }
            } else if let AxisChanged(axis, amt, _) = event {
                if axis == Axis::LeftStickX {
                    mouse_x_amt = if amt.abs() > DEADZONE {amt * MOUSE_SPEED_MODIFIER} else {0.0};
                } else if axis == Axis::LeftStickY {
                    mouse_y_amt = if amt.abs() > DEADZONE {-amt * MOUSE_SPEED_MODIFIER} else {0.0};
                }
            }
        }

        if mouse_x_amt.abs() > 0.0 || mouse_y_amt.abs() > 0.0 {
            mouse_x_pos += mouse_x_amt;
            mouse_y_pos += mouse_y_amt;
            enigo.move_mouse(mouse_x_pos as i32, mouse_y_pos as i32, Coordinate::Abs).unwrap();
        } else {
            let (mouse_x_pix, mouse_y_pix) = enigo.location().unwrap_or((mouse_x_pos as i32, mouse_y_pos as i32));
            mouse_x_pos = mouse_x_pix as f32;
            mouse_y_pos = mouse_y_pix as f32;
        }
    }
}