use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use enigo::{Coordinate, Direction, Enigo, Keyboard, Mouse, Settings};
use gilrs::{Axis, Event, Gilrs};
use gilrs::EventType::{AxisChanged, ButtonPressed, ButtonReleased};
use crate::backend::config_manager::ProfileConfig;
use crate::utils::lock_error_handler_string;

const DEADZONE: f32 = 0.05;
const MOUSE_SPEED_MODIFIER: f32 = 0.5;

pub async fn handle_controller_input(gilrs: Arc<Mutex<Gilrs>>, profile_config: Arc<Mutex<ProfileConfig>>, is_handler_running: Arc<AtomicBool>) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
    let active_gamepad_config_map = profile_config.lock().map_err(lock_error_handler_string)?.get_gamepad_config_map(gilrs.clone());

    let (mouse_x_pix, mouse_y_pix) = enigo.location().unwrap_or((0, 0));
    let mut mouse_x_pos = mouse_x_pix as f32;
    let mut mouse_y_pos = mouse_y_pix as f32;
    let mut mouse_x_amt = 0.0;
    let mut mouse_y_amt = 0.0;

    while is_handler_running.load(Ordering::Relaxed) == true {
        // Examine new events
        while let Some(Event { id, event, .. }) = gilrs.lock().map_err(lock_error_handler_string)?.next_event() {
            // TODO: Handle when Gilrs find a new GamepadId that's missing from the map!
            let agc = active_gamepad_config_map.get(&id).unwrap();

            match event {
                ButtonPressed(btn, _) => {
                    if let Some(key) = agc.get_key(&btn) {
                        enigo.key(*key, Direction::Press).map_err(|e| e.to_string())?;
                    }
                }
                ButtonReleased(btn, _) => {
                    if let Some(key) = agc.get_key(&btn) {
                        enigo.key(*key, Direction::Release).map_err(|e| e.to_string())?;
                    }
                }
                AxisChanged(axis, amt, _) => {
                    if axis == Axis::LeftStickX {
                        mouse_x_amt = if amt.abs() > DEADZONE {amt * MOUSE_SPEED_MODIFIER} else {0.0};
                    } else if axis == Axis::LeftStickY {
                        mouse_y_amt = if amt.abs() > DEADZONE {-amt * MOUSE_SPEED_MODIFIER} else {0.0};
                    }
                }
                _ => {}
            };
        }

        if mouse_x_amt.abs() > 0.0 || mouse_y_amt.abs() > 0.0 {
            mouse_x_pos += mouse_x_amt;
            mouse_y_pos += mouse_y_amt;
            enigo.move_mouse(mouse_x_pos as i32, mouse_y_pos as i32, Coordinate::Abs).map_err(|e| e.to_string())?;
        } else {
            let (mouse_x_pix, mouse_y_pix) = enigo.location().unwrap_or((mouse_x_pos as i32, mouse_y_pos as i32));
            mouse_x_pos = mouse_x_pix as f32;
            mouse_y_pos = mouse_y_pix as f32;
        }
    }
    Ok(())
}
