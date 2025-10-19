use std::time::SystemTime;
use enigo::{Coordinate, Enigo, Mouse, Settings};
use gilrs::{Axis, Event, Gilrs};
use gilrs::EventType::AxisChanged;


const DEADZONE: f32 = 0.05;
const MOUSE_SPEED_MODIFIER: f32 = 0.5;

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let (mouse_x_pix, mouse_y_pix) = enigo.location().unwrap();
    let mut mouse_x_pos = mouse_x_pix as f32;
    let mut mouse_y_pos = mouse_y_pix as f32;
    let mut mouse_x_amt = 0.0;
    let mut mouse_y_amt = 0.0;

    loop {
        // Examine new events
        while let Some(Event { id, event, time, .. }) = gilrs.next_event() {
            // println!("{:?} New event from {}: {:?}", time, id, event);

            if let AxisChanged(axis, amt, _) = event {
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
            let (mouse_x_pix, mouse_y_pix) = enigo.location().unwrap();
            mouse_x_pos = mouse_x_pix as f32;
            mouse_y_pos = mouse_y_pix as f32;
        }
    }
}
