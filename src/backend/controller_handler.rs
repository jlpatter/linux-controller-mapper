use crate::backend::config_manager::{GamepadConfig, ProfileConfig};
use crate::backend::key_utils::MouseButtonOrKey;
use crate::utils::lock_error_handler_string;
use anyhow::Result;
use enigo::{Coordinate, Direction, Enigo, Keyboard, Mouse, Settings};
use gilrs::EventType::{AxisChanged, ButtonPressed, ButtonReleased};
use gilrs::{Axis, Button, Event, Gilrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

const DEADZONE: f32 = 0.05;
const MOUSE_SPEED_MODIFIER: f32 = 0.5;

fn perform_key_press(
    enigo: &mut Enigo,
    agc: &GamepadConfig,
    btn: Button,
    dir: Direction,
) -> Result<(), String> {
    if let Some(mb_key) = agc.get_key(&btn) {
        if let MouseButtonOrKey::MouseButton(mb) = mb_key {
            enigo.button(*mb, dir).map_err(|e| e.to_string())?;
        } else if let MouseButtonOrKey::Key(key) = mb_key {
            enigo.key(*key, dir).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub async fn handle_controller_input(
    profile_config: Arc<Mutex<ProfileConfig>>,
    is_handler_running: Arc<AtomicBool>,
) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;
    let mut gilrs = Gilrs::new().map_err(|e| e.to_string())?;
    let active_gamepad_config_map = profile_config
        .lock()
        .map_err(lock_error_handler_string)?
        .get_gamepad_config_map(&gilrs);

    let (mouse_x_pix, mouse_y_pix) = enigo.location().unwrap_or((0, 0));
    let mut mouse_x_pos = mouse_x_pix as f32;
    let mut mouse_y_pos = mouse_y_pix as f32;
    let mut mouse_x_amt = 0.0;
    let mut mouse_y_amt = 0.0;

    while is_handler_running.load(Ordering::Relaxed) == true {
        // Examine new events
        while let Some(Event { id, event, .. }) = gilrs.next_event() {
            let agc = active_gamepad_config_map
                .get(&id)
                .ok_or("ERROR: Gamepad config couldn't be mapped to a Gamepad!")?;

            match event {
                ButtonPressed(btn, _) => {
                    perform_key_press(&mut enigo, agc, btn, Direction::Press)?;
                }
                ButtonReleased(btn, _) => {
                    perform_key_press(&mut enigo, agc, btn, Direction::Release)?;
                }
                AxisChanged(axis, amt, _) => {
                    if axis == Axis::LeftStickX {
                        mouse_x_amt = if amt.abs() > DEADZONE {
                            amt * MOUSE_SPEED_MODIFIER
                        } else {
                            0.0
                        };
                    } else if axis == Axis::LeftStickY {
                        mouse_y_amt = if amt.abs() > DEADZONE {
                            -amt * MOUSE_SPEED_MODIFIER
                        } else {
                            0.0
                        };
                    }
                }
                _ => {}
            };
        }

        if mouse_x_amt.abs() > 0.0 || mouse_y_amt.abs() > 0.0 {
            mouse_x_pos += mouse_x_amt;
            mouse_y_pos += mouse_y_amt;
            enigo
                .move_mouse(mouse_x_pos as i32, mouse_y_pos as i32, Coordinate::Abs)
                .map_err(|e| e.to_string())?;
        } else {
            let (mouse_x_pix, mouse_y_pix) = enigo
                .location()
                .unwrap_or((mouse_x_pos as i32, mouse_y_pos as i32));
            mouse_x_pos = mouse_x_pix as f32;
            mouse_y_pos = mouse_y_pix as f32;
        }
    }
    Ok(())
}
